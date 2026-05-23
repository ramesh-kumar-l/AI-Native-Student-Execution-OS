/// Ollama provider — local LLM via Ollama HTTP API (ADR-0002 D-4).
/// Ollama API reference: https://github.com/ollama/ollama/blob/main/docs/api.md
use async_trait::async_trait;
use futures::stream::{self, BoxStream, StreamExt};
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

use crate::ai::task::{ChatMessage, ChatOptions, MessageRole, StreamChunk, TokenUsage};
use crate::error::ProviderError;

use super::Provider;

pub struct OllamaProvider {
    client: reqwest::Client,
    base_url: String,
}

impl OllamaProvider {
    pub fn new(base_url: String) -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(120))
                .build()
                .expect("reqwest client construction never fails with these opts"),
            base_url,
        }
    }
}

// ── Ollama wire types ─────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
struct OllamaChatRequest {
    model: String,
    messages: Vec<OllamaMessage>,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<OllamaOptions>,
}

#[derive(Debug, Serialize)]
struct OllamaMessage {
    role: &'static str,
    content: String,
}

#[derive(Debug, Serialize)]
struct OllamaOptions {
    temperature: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    seed: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_predict: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct OllamaChatChunk {
    message: OllamaChunkMessage,
    done: bool,
    #[serde(default)]
    prompt_eval_count: Option<u32>,
    #[serde(default)]
    eval_count: Option<u32>,
    model: String,
}

#[derive(Debug, Deserialize)]
struct OllamaChunkMessage {
    content: String,
}

fn role_str(role: &MessageRole) -> &'static str {
    match role {
        MessageRole::System => "system",
        MessageRole::User => "user",
        MessageRole::Assistant => "assistant",
    }
}

// ── Ollama embed types ────────────────────────────────────────────────────────

#[derive(Serialize)]
struct OllamaEmbedRequest {
    model: String,
    input: Vec<String>,
}

#[derive(Deserialize)]
struct OllamaEmbedResponse {
    embeddings: Vec<Vec<f32>>,
}

// ── Provider impl ─────────────────────────────────────────────────────────────

#[async_trait]
impl Provider for OllamaProvider {
    fn name(&self) -> &str {
        "ollama"
    }

    async fn is_available(&self) -> bool {
        let url = format!("{}/api/version", self.base_url);
        match self.client.get(&url).send().await {
            Ok(resp) => resp.status().is_success(),
            Err(_) => false,
        }
    }

    async fn chat_stream(
        &self,
        messages: &[ChatMessage],
        opts: &ChatOptions,
    ) -> Result<BoxStream<'static, Result<StreamChunk, ProviderError>>, ProviderError> {
        let url = format!("{}/api/chat", self.base_url);
        let body = OllamaChatRequest {
            model: opts.model.clone(),
            messages: messages
                .iter()
                .map(|m| OllamaMessage {
                    role: role_str(&m.role),
                    content: m.content.clone(),
                })
                .collect(),
            stream: true,
            options: Some(OllamaOptions {
                temperature: opts.temperature,
                seed: opts.seed,
                num_predict: opts.max_tokens,
            }),
        };

        debug!(model = %opts.model, "sending chat request to Ollama");

        let response = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(ProviderError::from)?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            warn!(status = %status, body = %text, "Ollama returned error");
            return Err(ProviderError::Http {
                message: format!("Ollama {status}: {text}"),
            });
        }

        // Ollama streams NDJSON — one JSON object per line.
        let byte_stream = response.bytes_stream();
        let model_name = opts.model.clone();

        let chunk_stream = byte_stream.flat_map(move |result| {
            let model = model_name.clone();
            match result {
                Err(e) => stream::iter(vec![Err(ProviderError::Http {
                    message: e.to_string(),
                })]),
                Ok(bytes) => {
                    let lines: Vec<_> = String::from_utf8_lossy(&bytes)
                        .lines()
                        .filter(|l| !l.trim().is_empty())
                        .filter_map(|line| {
                            match serde_json::from_str::<OllamaChatChunk>(line) {
                                Ok(chunk) => {
                                    let usage = if chunk.done {
                                        Some(TokenUsage {
                                            prompt_tokens: chunk.prompt_eval_count.unwrap_or(0),
                                            completion_tokens: chunk.eval_count.unwrap_or(0),
                                        })
                                    } else {
                                        None
                                    };
                                    Some(Ok(StreamChunk {
                                        content: chunk.message.content,
                                        done: chunk.done,
                                        model: Some(chunk.model.clone()),
                                        provider: Some("ollama".to_string()),
                                        usage,
                                    }))
                                }
                                Err(e) => {
                                    warn!(err = %e, "failed to parse Ollama chunk");
                                    None
                                }
                            }
                        })
                        .collect();
                    stream::iter(lines)
                }
            }
        });

        Ok(Box::pin(chunk_stream))
    }

    async fn embed(
        &self,
        texts: &[String],
        model: &str,
    ) -> Result<Vec<Vec<f32>>, ProviderError> {
        let url = format!("{}/api/embed", self.base_url);
        let body = OllamaEmbedRequest {
            model: model.to_string(),
            input: texts.to_vec(),
        };
        let resp: OllamaEmbedResponse = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(ProviderError::from)?
            .json()
            .await
            .map_err(ProviderError::from)?;
        Ok(resp.embeddings)
    }
}
