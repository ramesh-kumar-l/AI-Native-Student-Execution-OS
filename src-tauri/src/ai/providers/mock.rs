/// Mock provider used exclusively in tests (`DAEMON_TEST_MODE=1`).
/// Returns a deterministic canned response so CI never needs a running Ollama or cloud key.
use async_trait::async_trait;
use futures::stream::{self, BoxStream};

use crate::ai::task::{ChatMessage, ChatOptions, StreamChunk, TokenUsage};
use crate::error::ProviderError;

use super::Provider;

pub struct MockProvider {
    pub name: &'static str,
    pub response_words: Vec<&'static str>,
}

impl Default for MockProvider {
    fn default() -> Self {
        Self {
            name: "mock",
            response_words: vec![
                "Hello!",
                " I'm",
                " your",
                " AI",
                " mentor.",
                " How",
                " can",
                " I",
                " help",
                " you",
                " today?",
            ],
        }
    }
}

#[async_trait]
impl Provider for MockProvider {
    fn name(&self) -> &str {
        self.name
    }

    async fn is_available(&self) -> bool {
        true
    }

    async fn chat_stream(
        &self,
        _messages: &[ChatMessage],
        _opts: &ChatOptions,
    ) -> Result<BoxStream<'static, Result<StreamChunk, ProviderError>>, ProviderError> {
        let words = self.response_words.clone();
        let n = words.len();
        let chunks: Vec<Result<StreamChunk, ProviderError>> = words
            .into_iter()
            .enumerate()
            .map(|(i, word)| {
                let done = i == n - 1;
                Ok(StreamChunk {
                    content: word.to_string(),
                    done,
                    model: Some("mock-model".to_string()),
                    provider: Some("mock".to_string()),
                    usage: if done {
                        Some(TokenUsage {
                            prompt_tokens: 10,
                            completion_tokens: 11,
                        })
                    } else {
                        None
                    },
                })
            })
            .collect();

        Ok(Box::pin(stream::iter(chunks)))
    }

    async fn embed(
        &self,
        texts: &[String],
        _model: &str,
    ) -> Result<Vec<Vec<f32>>, ProviderError> {
        // Return a fixed-length zero vector per input — enough for L2 API tests.
        Ok(texts.iter().map(|_| vec![0.0f32; 768]).collect())
    }
}
