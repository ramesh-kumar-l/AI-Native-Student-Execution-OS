use crate::ai::task::RoutingPreference;
use serde::{Deserialize, Serialize};

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
