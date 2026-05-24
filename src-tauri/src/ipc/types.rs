use crate::ai::task::RoutingPreference;
use serde::{Deserialize, Serialize};

// ── Project request types ─────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProjectRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
}

// ── Task request types ────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: Option<String>,
    #[serde(default = "default_priority")]
    pub priority: i32,
    pub due_at: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTaskRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub priority: Option<i32>,
    pub due_at: Option<i64>,
}

fn default_priority() -> i32 {
    1
}

// ── Query param types ─────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct LimitQuery {
    #[serde(default = "default_limit")]
    pub limit: u32,
}

fn default_limit() -> u32 {
    50
}

/// Inbound request for a single mentor interaction turn.
#[derive(Debug, Deserialize)]
pub struct MentorTurnRequest {
    pub message: String,
    pub conversation_id: Option<String>,
    pub project_id: Option<String>,
    #[serde(default)]
    pub routing: RoutingPreference,
}

/// One SSE event chunk sent back to the client during streaming.
#[derive(Debug, Serialize)]
pub struct MentorTurnChunk {
    pub chunk: String,
    pub done: bool,
    pub correlation_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Response from `GET /api/v1/health`.
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub version: &'static str,
    pub daemon_ready: bool,
}

/// Response from `GET /api/v1/audit/recent`.
#[derive(Debug, Serialize)]
pub struct AuditEntryDto {
    pub id: String,
    pub event_type: String,
    pub correlation_id: String,
    pub created_at: i64,
    pub metadata: Option<serde_json::Value>,
}

/// Generic API error response body.
#[derive(Debug, Serialize)]
pub struct ApiError {
    pub error: String,
    pub correlation_id: Option<String>,
}

// ── Phase 3 · Workflow signal types ──────────────────────────────────────────

/// Inbound signal sent over WebSocket by the VSCode extension.
#[derive(Debug, Deserialize)]
pub struct IncomingSignal {
    pub signal_type: String,
    pub project_id: Option<String>,
    pub file_path: Option<String>,
    pub language: Option<String>,
    #[serde(default)]
    pub payload: serde_json::Value,
}

/// Query params for `GET /api/v1/signals`.
#[derive(Debug, Deserialize)]
pub struct SignalQuery {
    pub project_id: Option<String>,
    #[serde(default = "default_signal_limit")]
    pub limit: u32,
}

fn default_signal_limit() -> u32 {
    50
}
