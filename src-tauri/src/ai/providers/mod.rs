use async_trait::async_trait;
use futures::stream::BoxStream;

use crate::ai::task::{ChatMessage, ChatOptions, StreamChunk};
use crate::error::ProviderError;

pub mod claude;
pub mod mock;
pub mod ollama;

/// The provider abstraction contract (ADR-0002).
/// No engine or orchestrator code ever imports a concrete provider — only this trait.
#[async_trait]
pub trait Provider: Send + Sync {
    /// Short stable name used in routing decisions and audit logs.
    fn name(&self) -> &str;

    /// Whether the provider endpoint is reachable right now.
    async fn is_available(&self) -> bool;

    /// Stream a chat completion. Returns a `BoxStream` of chunks.
    /// The final chunk has `done = true` and optional `usage`.
    async fn chat_stream(
        &self,
        messages: &[ChatMessage],
        opts: &ChatOptions,
    ) -> Result<BoxStream<'static, Result<StreamChunk, ProviderError>>, ProviderError>;

    /// Embed a batch of texts. Returns one `Vec<f32>` per input.
    async fn embed(
        &self,
        texts: &[String],
        model: &str,
    ) -> Result<Vec<Vec<f32>>, ProviderError>;
}

/// Registry of known providers, queried by the orchestrator.
pub struct ProviderRegistry {
    local: Vec<Box<dyn Provider>>,
    cloud: Vec<Box<dyn Provider>>,
}

impl ProviderRegistry {
    pub fn new(local: Vec<Box<dyn Provider>>, cloud: Vec<Box<dyn Provider>>) -> Self {
        Self { local, cloud }
    }

    pub fn local_providers(&self) -> &[Box<dyn Provider>] {
        &self.local
    }

    pub fn cloud_providers(&self) -> &[Box<dyn Provider>] {
        &self.cloud
    }

    /// Find the first available local provider.
    pub async fn first_available_local(&self) -> Option<&dyn Provider> {
        for p in &self.local {
            if p.is_available().await {
                return Some(p.as_ref());
            }
        }
        None
    }

    /// Find the first available cloud provider.
    pub async fn first_available_cloud(&self) -> Option<&dyn Provider> {
        for p in &self.cloud {
            if p.is_available().await {
                return Some(p.as_ref());
            }
        }
        None
    }
}
