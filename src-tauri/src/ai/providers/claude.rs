/// Anthropic Claude provider — cloud LLM via Anthropic Messages API.
/// Uses SSE streaming. ADR-0002: single cloud provider for Phase 1.
use async_trait::async_trait;
use futures::stream::{self, BoxStream, StreamExt};
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

use crate::ai::task::{ChatMessage, ChatOptions, MessageRole, StreamChunk, TokenUsage};
use crate::error::ProviderError;

use super::Provider;

const ANTHROPIC_API_BASE: &str = "https://api.anthropic.com/v1";
const ANTHROPIC_VERSION: &str = "2023-06-01";

pub struct ClaudeProvider {
    client: reqwest::Client,
    api_key: String,
}

impl ClaudeProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(180))
                .build()
                .expect("reqwest client construction is infallible"),
            api_key,
        }
    }
}

// ── Claude wire types ─────────────────────────────────────────────────────────

#[derive(Serialize)]
struct ClaudeRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<ClaudeMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<String>,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f64>,
}

#[derive(Serialize)]
struct ClaudeMessage {
    role: &'static str,
    content: String,
}

/// Subset of Anthropic SSE delta events we care about.
#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ClaudeEvent {
    MessageStart {
        message: ClaudeMessageStart,
    },
    ContentBlockDelta {
        delta: ContentDelta,
    },
    MessageDelta {
        usage: Option<ClaudeUsage>,
    },
    MessageStop,
    #[serde(other)]
    Other,
}

#[derive(Deserialize)]
struct ClaudeMessageStart {
    model: String,
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ContentDelta {
    TextDelta { text: String },
    #[serde(other)]
    Other,
}

#[derive(Deserialize)]
struct ClaudeUsage {
    input_tokens: Option<u32>,
    output_tokens: Option<u32>,
}

fn to_claude_role(role: &MessageRole) -> &'static str {
    match role {
        MessageRole::User | MessageRole::System => "user",
        MessageRole::Assistant => "assistant",
    }
}

// ── Provider impl ─────────────────────────────────────────────────────────────

#[async_trait]
impl Provider for ClaudeProvider {
    fn name(&self) -> &str {
        "claude"
    }

    async fn is_available(&self) -> bool {
        // Claude doesn't have a free health endpoint; we check if we have a key.
        !self.api_key.is_empty()
    }

    async fn chat_stream(
        &self,
        messages: &[ChatMessage],
        opts: &ChatOptions,
    ) -> Result<BoxStream<'static, Result<StreamChunk, ProviderError>>, ProviderError> {
        let system_msg = messages
            .iter()
            .find(|m| m.role == MessageRole::System)
            .map(|m| m.content.clone());

        let claude_messages: Vec<ClaudeMessage> = messages
            .iter()
            .filter(|m| m.role != MessageRole::System)
            .map(|m| ClaudeMessage {
                role: to_claude_role(&m.role),
                content: m.content.clone(),
            })
            .collect();

        let body = ClaudeRequest {
            model: opts.model.clone(),
            max_tokens: opts.max_tokens.unwrap_or(4096),
            messages: claude_messages,
            system: system_msg,
            stream: true,
            temperature: if (opts.temperature - 1.0).abs() > f32::EPSILON {
                Some(opts.temperature as f64)
            } else {
                None
            },
        };

        debug!(model = %opts.model, "sending chat request to Claude");

        let response = self
            .client
            .post(format!("{ANTHROPIC_API_BASE}/messages"))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", ANTHROPIC_VERSION)
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(ProviderError::from)?;

        if !response.status().is_success() {
            let status = response.status();
            if status.as_u16() == 429 {
                return Err(ProviderError::RateLimited {
                    provider: "claude".to_string(),
                });
            }
            if status.as_u16() == 401 {
                return Err(ProviderError::AuthRequired {
                    provider: "claude".to_string(),
                });
            }
            let text = response.text().await.unwrap_or_default();
            return Err(ProviderError::Http {
                message: format!("Claude {status}: {text}"),
            });
        }

        let byte_stream = response.bytes_stream();
        let model_name = opts.model.clone();
        // Track model name across SSE events via message_start.
        let mut resolved_model = model_name.clone();

        let chunk_stream = byte_stream.flat_map(move |result| {
            match result {
                Err(e) => stream::iter(vec![Err(ProviderError::Http {
                    message: e.to_string(),
                })]),
                Ok(bytes) => {
                    let text = String::from_utf8_lossy(&bytes);
                    let mut chunks = vec![];
                    for line in text.lines() {
                        let line = line.trim();
                        if line.starts_with("data: ") {
                            let json_str = &line["data: ".len()..];
                            if json_str == "[DONE]" {
                                continue;
                            }
                            match serde_json::from_str::<ClaudeEvent>(json_str) {
                                Ok(ClaudeEvent::MessageStart { message }) => {
                                    resolved_model = message.model.clone();
                                }
                                Ok(ClaudeEvent::ContentBlockDelta { delta }) => {
                                    if let ContentDelta::TextDelta { text } = delta {
                                        chunks.push(Ok(StreamChunk {
                                            content: text,
                                            done: false,
                                            model: Some(resolved_model.clone()),
                                            provider: Some("claude".to_string()),
                                            usage: None,
                                        }));
                                    }
                                }
                                Ok(ClaudeEvent::MessageDelta { usage }) => {
                                    let token_usage = usage.map(|u| TokenUsage {
                                        prompt_tokens: u.input_tokens.unwrap_or(0),
                                        completion_tokens: u.output_tokens.unwrap_or(0),
                                    });
                                    chunks.push(Ok(StreamChunk {
                                        content: String::new(),
                                        done: true,
                                        model: Some(resolved_model.clone()),
                                        provider: Some("claude".to_string()),
                                        usage: token_usage,
                                    }));
                                }
                                Ok(ClaudeEvent::MessageStop | ClaudeEvent::Other) => {}
                                Err(e) => {
                                    warn!(err = %e, line = line, "failed to parse Claude SSE event");
                                }
                            }
                        }
                    }
                    stream::iter(chunks)
                }
            }
        });

        Ok(Box::pin(chunk_stream))
    }

    async fn embed(
        &self,
        _texts: &[String],
        _model: &str,
    ) -> Result<Vec<Vec<f32>>, ProviderError> {
        // Anthropic doesn't expose an embeddings endpoint. Embeddings always use local provider.
        Err(ProviderError::Unavailable {
            provider: "claude (embed — not supported; use local provider)".to_string(),
        })
    }
}
