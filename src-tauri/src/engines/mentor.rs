/// Mentor engine — orchestrates a single mentor interaction turn.
/// Assembles context from memory, creates a TaskSpec, calls the orchestrator,
/// persists the assembled response to L0+L1, emits audit records.
use chrono::{DateTime, Utc};
use futures::StreamExt;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tracing::{error, info, instrument};

use crate::ai::task::{
    AuditContext, ChatMessage, ContextChunk, DeterminismPolicy, MessageRole, RoutingPolicy,
    RoutingPreference, StreamChunk, TaskInputs, TaskKind, TaskSpec,
};
use crate::error::{DaemonError, ProviderError};
use crate::memory::l1::L1Store;
use crate::memory::MemoryStore;
use crate::memory::l0::EventKind;
use crate::observability::audit::{AuditLog, AuditRecord};
use crate::ai::Orchestrator;
use futures::stream::BoxStream;

pub struct MentorEngine {
    memory: Arc<MemoryStore>,
    orchestrator: Arc<Orchestrator>,
    audit: Arc<AuditLog>,
}

pub struct MentorTurnInput {
    pub message: String,
    pub conversation_id: Option<String>,
    pub project_id: Option<String>,
    pub routing: RoutingPreference,
    pub correlation_id: String,
    pub user_id: String,
}

pub struct MentorTurnOutput {
    pub conversation_id: String,
    pub stream: BoxStream<'static, Result<StreamChunk, ProviderError>>,
}

impl MentorEngine {
    pub fn new(
        memory: Arc<MemoryStore>,
        orchestrator: Arc<Orchestrator>,
        audit: Arc<AuditLog>,
    ) -> Self {
        Self { memory, orchestrator, audit }
    }

    #[instrument(skip(self, input), fields(corr = %input.correlation_id))]
    pub async fn process_turn(
        &self,
        input: MentorTurnInput,
    ) -> Result<MentorTurnOutput, DaemonError> {
        // 1. Resolve or create conversation.
        let conversation_id = match &input.conversation_id {
            Some(id) => match self.memory.l1.get_conversation(id).await? {
                Some(_) => id.clone(),
                None => self.create_conversation(&input).await?,
            },
            None => self.create_conversation(&input).await?,
        };

        // 2. Emit L0 event + audit.
        self.memory.l0.append_typed(
            &EventKind::MentorTurnReceived {
                conversation_id: conversation_id.clone(),
                message_preview: input.message.chars().take(80).collect(),
            },
            &input.correlation_id,
            &input.user_id,
        ).await?;

        self.audit.record(AuditRecord::mentor_turn_received(
            &conversation_id,
            &input.correlation_id,
            &input.user_id,
        )).await.ok();

        // 3. Persist the user's message to L1.
        let user_msg = L1Store::new_message(
            &conversation_id, "user", &input.message,
            None, None, Some(input.correlation_id.clone()),
        );
        self.memory.l1.insert_message(user_msg).await?;

        // 4. Load conversation history for context window.
        let history = self.memory.l1
            .messages_for_conversation(&conversation_id, 20).await?;

        let mut chat_messages: Vec<ChatMessage> = history.iter().map(|m| ChatMessage {
            role: match m.role.as_str() {
                "assistant" => MessageRole::Assistant,
                "system" => MessageRole::System,
                _ => MessageRole::User,
            },
            content: m.content.clone(),
        }).collect();

        // 4b. Inject recent workflow signals as a system message when a project is set.
        //     This gives the AI live context about what the user is actively working on.
        if let Some(ref project_id) = input.project_id {
            let signals = self.memory.l1
                .recent_signals(Some(project_id), 10).await
                .unwrap_or_default();
            if !signals.is_empty() {
                let lines: Vec<String> = signals.iter().map(|s| {
                    let ts = DateTime::<Utc>::from_timestamp_millis(s.created_at)
                        .map(|dt| dt.format("%H:%M").to_string())
                        .unwrap_or_default();
                    let file = s.file_path.as_deref().unwrap_or("?");
                    let lang = s.language.as_deref()
                        .map(|l| format!(" ({})", l))
                        .unwrap_or_default();
                    format!("- [{}] {} → {}{}", ts, s.signal_type, file, lang)
                }).collect();
                let ctx = format!(
                    "Current workflow context (recent VSCode activity, newest last):\n{}",
                    lines.join("\n")
                );
                // Prepend as system message so it seats before the conversation history.
                chat_messages.insert(0, ChatMessage {
                    role: MessageRole::System,
                    content: ctx,
                });
            }
        }

        // 5. Build TaskSpec.
        let task = TaskSpec {
            task_kind: TaskKind::MentorTurn,
            inputs: TaskInputs {
                messages: chat_messages,
                conversation_id: Some(conversation_id.clone()),
                project_id: input.project_id.clone(),
                context: vec![ContextChunk {
                    source: "l1:conversation_history".to_string(),
                    content: format!("{} messages in context", history.len()),
                    relevance_score: 1.0,
                }],
            },
            routing_policy: RoutingPolicy {
                preferred: input.routing.clone(),
                ..Default::default()
            },
            determinism: DeterminismPolicy::default(),
            audit: AuditContext {
                user_id: input.user_id.clone(),
                correlation_id: input.correlation_id.clone(),
            },
        };

        // 6. Run through orchestrator.
        let raw_stream = self.orchestrator.run(&task).await?;

        // 7. Spawn a task that reads the full stream, accumulates the response,
        //    persists it to L1, and re-broadcasts each chunk via a channel.
        //    This way the stream is non-blocking for the HTTP handler while
        //    persistence happens correctly after the final chunk.
        let (tx, rx) = mpsc::channel::<Result<StreamChunk, ProviderError>>(128);
        let memory = Arc::clone(&self.memory);
        let audit = Arc::clone(&self.audit);
        let conv_id = conversation_id.clone();
        let corr_id = input.correlation_id.clone();
        let user_id = input.user_id.clone();

        tokio::spawn(async move {
            let mut stream = raw_stream;
            let mut accumulated = String::new();
            let mut last_model: Option<String> = None;
            let mut last_provider: Option<String> = None;

            while let Some(result) = stream.next().await {
                let done = result.as_ref().map(|c| c.done).unwrap_or(false);

                if let Ok(ref chunk) = result {
                    accumulated.push_str(&chunk.content);
                    if chunk.model.is_some() { last_model = chunk.model.clone(); }
                    if chunk.provider.is_some() { last_provider = chunk.provider.clone(); }
                }

                // Forward to the HTTP handler; stop if the receiver was dropped.
                if tx.send(result).await.is_err() {
                    break;
                }

                if done {
                    // Persist the assembled assistant message.
                    if !accumulated.is_empty() {
                        let assistant_msg = L1Store::new_message(
                            &conv_id, "assistant", &accumulated,
                            last_model.clone(), last_provider.clone(),
                            Some(corr_id.clone()),
                        );
                        let msg_id = assistant_msg.id.clone();

                        if let Err(e) = memory.l1.insert_message(assistant_msg).await {
                            error!(err = %e, "failed to persist assistant message");
                        }
                        if let Err(e) = memory.l1.touch_conversation(&conv_id).await {
                            error!(err = %e, "failed to touch conversation timestamp");
                        }
                        memory.l0.append_typed(
                            &EventKind::MessageWritten {
                                message_id: msg_id.clone(),
                                conversation_id: conv_id.clone(),
                                role: "assistant".to_string(),
                            },
                            &corr_id, &user_id,
                        ).await.ok();
                        audit.record(AuditRecord::mentor_turn_completed(
                            &conv_id, &msg_id, &corr_id, &user_id,
                        )).await.ok();
                    }
                    break;
                }
            }
        });

        info!(
            conversation_id = %conversation_id,
            correlation_id = %input.correlation_id,
            "mentor turn stream started"
        );

        Ok(MentorTurnOutput {
            conversation_id,
            stream: Box::pin(ReceiverStream::new(rx)),
        })
    }

    async fn create_conversation(
        &self,
        input: &MentorTurnInput,
    ) -> Result<String, DaemonError> {
        let conv = self.memory.l1.create_conversation(input.project_id.clone()).await?;
        self.memory.l0.append_typed(
            &EventKind::ConversationCreated { conversation_id: conv.id.clone() },
            &input.correlation_id,
            &input.user_id,
        ).await?;
        Ok(conv.id)
    }
}
