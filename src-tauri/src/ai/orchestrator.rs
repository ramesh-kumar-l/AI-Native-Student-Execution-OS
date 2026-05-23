/// AI orchestrator — routes TaskSpec to the right provider, enforces audit.
/// Engines call `orchestrator.run(task)` and never touch a provider directly.
use futures::stream::BoxStream;
use futures::StreamExt;
use std::sync::Arc;
use tracing::{info, warn};

use crate::ai::task::{
    ChatOptions, RoutingPreference, StreamChunk, TaskSpec,
};
use crate::error::{DaemonError, ProviderError};
use crate::observability::audit::{AuditLog, AuditRecord};

use super::providers::ProviderRegistry;

pub struct Orchestrator {
    registry: Arc<ProviderRegistry>,
    audit: Arc<AuditLog>,
}

impl Orchestrator {
    pub fn new(registry: Arc<ProviderRegistry>, audit: Arc<AuditLog>) -> Self {
        Self { registry, audit }
    }

    /// Route the task and return a streaming response.
    /// Emits routing decision + completion events to the audit log.
    pub async fn run(
        &self,
        task: &TaskSpec,
    ) -> Result<BoxStream<'static, Result<StreamChunk, ProviderError>>, DaemonError> {
        let provider = self.resolve_provider(task).await?;

        let opts = ChatOptions {
            model: self.default_model_for(provider.name()),
            temperature: task.determinism.temperature,
            seed: task.determinism.seed,
            max_tokens: Some(4096),
        };

        info!(
            provider = provider.name(),
            model = %opts.model,
            task_kind = ?task.task_kind,
            correlation_id = %task.audit.correlation_id,
            "routing AI call"
        );

        self.audit
            .record(AuditRecord::ai_call_started(
                provider.name(),
                &opts.model,
                &task.audit.correlation_id,
                &task.audit.user_id,
            ))
            .await
            .ok(); // audit failure must never fail the main request

        let stream = provider
            .chat_stream(&task.inputs.messages, &opts)
            .await
            .map_err(DaemonError::Provider)?;

        let audit = Arc::clone(&self.audit);
        let correlation_id = task.audit.correlation_id.clone();
        let user_id = task.audit.user_id.clone();
        let provider_name = provider.name().to_string();
        let model_name = opts.model.clone();

        // Wrap the stream to record completion on the final chunk.
        let wrapped = stream.map(move |result| {
            if let Ok(ref chunk) = result {
                if chunk.done {
                    let usage_prompt = chunk.usage.as_ref().map(|u| u.prompt_tokens).unwrap_or(0);
                    let usage_completion =
                        chunk.usage.as_ref().map(|u| u.completion_tokens).unwrap_or(0);
                    let audit = Arc::clone(&audit);
                    let corr = correlation_id.clone();
                    let uid = user_id.clone();
                    let prov = provider_name.clone();
                    let model = model_name.clone();
                    tokio::spawn(async move {
                        audit
                            .record(AuditRecord::ai_call_completed(
                                &prov,
                                &model,
                                usage_prompt,
                                usage_completion,
                                &corr,
                                &uid,
                            ))
                            .await
                            .ok();
                    });
                }
            }
            result
        });

        Ok(Box::pin(wrapped))
    }

    async fn resolve_provider(
        &self,
        task: &TaskSpec,
    ) -> Result<&dyn super::providers::Provider, DaemonError> {
        let preferred = &task.routing_policy.preferred;

        match preferred {
            RoutingPreference::Local | RoutingPreference::Auto => {
                if let Some(p) = self.registry.first_available_local().await {
                    return Ok(p);
                }
                if *preferred == RoutingPreference::Auto {
                    if let Some(p) = self.registry.first_available_cloud().await {
                        warn!(
                            correlation_id = %task.audit.correlation_id,
                            "no local provider available, falling back to cloud"
                        );
                        return Ok(p);
                    }
                }
                Err(DaemonError::Provider(ProviderError::Unavailable {
                    provider: "local".to_string(),
                }))
            }
            RoutingPreference::Cloud => {
                if let Some(p) = self.registry.first_available_cloud().await {
                    return Ok(p);
                }
                Err(DaemonError::Provider(ProviderError::Unavailable {
                    provider: "cloud".to_string(),
                }))
            }
        }
    }

    fn default_model_for(&self, provider_name: &str) -> String {
        match provider_name {
            "ollama" => "llama3.2".to_string(),
            "claude" => "claude-sonnet-4-6".to_string(),
            "mock" => "mock-model".to_string(),
            other => other.to_string(),
        }
    }
}
