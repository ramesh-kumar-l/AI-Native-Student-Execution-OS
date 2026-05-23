use serde::{Deserialize, Serialize};

/// The contract between engines and the AI orchestrator.
/// Engines never specify a model name — only what the task needs.
/// The orchestrator decides routing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskSpec {
    pub task_kind: TaskKind,
    pub inputs: TaskInputs,
    pub routing_policy: RoutingPolicy,
    pub determinism: DeterminismPolicy,
    pub audit: AuditContext,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TaskKind {
    MentorTurn,
    CapabilityScore,
    ArtifactDraft,
    Summarize,
    Embed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskInputs {
    pub messages: Vec<ChatMessage>,
    pub conversation_id: Option<String>,
    pub project_id: Option<String>,
    /// Context chunks assembled by the orchestrator from memory layers.
    pub context: Vec<ContextChunk>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextChunk {
    /// Where this context came from (e.g., "l4:weekly_brief", "l2:vector_search").
    pub source: String,
    pub content: String,
    pub relevance_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingPolicy {
    pub preferred: RoutingPreference,
    pub quality_floor: QualityFloor,
    pub latency_budget_ms: Option<u64>,
}

impl Default for RoutingPolicy {
    fn default() -> Self {
        Self {
            preferred: RoutingPreference::Local,
            quality_floor: QualityFloor::Medium,
            latency_budget_ms: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RoutingPreference {
    #[default]
    Local,
    Cloud,
    Auto,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum QualityFloor {
    Low,
    #[default]
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeterminismPolicy {
    pub temperature: f32,
    pub seed: Option<u64>,
    pub cache_key: Option<String>,
}

impl Default for DeterminismPolicy {
    fn default() -> Self {
        Self {
            temperature: 0.7,
            seed: None,
            cache_key: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditContext {
    pub user_id: String,
    pub correlation_id: String,
}

/// One streamed token chunk from a provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamChunk {
    pub content: String,
    pub done: bool,
    pub model: Option<String>,
    pub provider: Option<String>,
    /// Only present on the final chunk when `done = true`.
    pub usage: Option<TokenUsage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
}

/// Options passed to the provider for a chat call.
#[derive(Debug, Clone)]
pub struct ChatOptions {
    pub model: String,
    pub temperature: f32,
    pub seed: Option<u64>,
    pub max_tokens: Option<u32>,
}
