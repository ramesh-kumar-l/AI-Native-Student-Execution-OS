/// Axum route handlers for the HTTP loopback IPC server.
/// Surfaces: Desktop app + VSCode extension both speak this API.
use axum::{
    extract::State,
    http::StatusCode,
    response::{
        sse::{Event, KeepAlive, Sse},
        IntoResponse, Json,
    },
};
use futures::stream::StreamExt;
use std::convert::Infallible;
use tracing::error;
use ulid::Ulid;

use crate::{
    ai::task::RoutingPreference,
    engines::mentor::MentorTurnInput,
    ipc::types::{AuditEntryDto, ApiError, HealthResponse, MentorTurnChunk, MentorTurnRequest},
    AppState,
};

pub async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
        daemon_ready: true,
    })
}

/// POST /api/v1/mentor/turn
/// Accepts a user message, streams back SSE chunks from the AI provider.
pub async fn mentor_turn(
    State(state): State<AppState>,
    Json(req): Json<MentorTurnRequest>,
) -> impl IntoResponse {
    let correlation_id = Ulid::new().to_string();
    let conv_id_for_header = req.conversation_id.clone();

    let input = MentorTurnInput {
        message: req.message,
        conversation_id: req.conversation_id,
        project_id: req.project_id,
        routing: req.routing,
        correlation_id: correlation_id.clone(),
        user_id: "local".to_string(),
    };

    match state.mentor.process_turn(input).await {
        Err(e) => {
            error!(err = %e, corr = %correlation_id, "mentor turn failed before stream");
            let body = ApiError {
                error: e.to_string(),
                correlation_id: Some(correlation_id.clone()),
            };
            // Return an SSE "error" event so the client can parse it uniformly.
            let chunk = MentorTurnChunk {
                chunk: String::new(),
                done: true,
                correlation_id,
                conversation_id: None,
                error: Some(body.error),
            };
            let event_data = serde_json::to_string(&chunk).unwrap_or_default();
            let sse = Sse::new(futures::stream::once(async move {
                Ok::<Event, Infallible>(Event::default().data(event_data))
            }));
            (StatusCode::INTERNAL_SERVER_ERROR, sse).into_response()
        }
        Ok(output) => {
            let conversation_id = output.conversation_id.clone();
            let corr = correlation_id.clone();

            let sse_stream = output.stream.map(move |result| {
                let chunk = match result {
                    Ok(sc) => MentorTurnChunk {
                        chunk: sc.content,
                        done: sc.done,
                        correlation_id: corr.clone(),
                        conversation_id: Some(conversation_id.clone()),
                        error: None,
                    },
                    Err(e) => MentorTurnChunk {
                        chunk: String::new(),
                        done: true,
                        correlation_id: corr.clone(),
                        conversation_id: Some(conversation_id.clone()),
                        error: Some(e.to_string()),
                    },
                };
                let data = serde_json::to_string(&chunk).unwrap_or_default();
                Ok::<Event, Infallible>(Event::default().data(data))
            });

            Sse::new(sse_stream)
                .keep_alive(KeepAlive::default())
                .into_response()
        }
    }
}

/// GET /api/v1/audit/recent?limit=50
pub async fn audit_recent(State(state): State<AppState>) -> impl IntoResponse {
    match state.audit.recent(50).await {
        Ok(entries) => Json(entries).into_response(),
        Err(e) => {
            error!(err = %e, "audit recent query failed");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError {
                    error: e.to_string(),
                    correlation_id: None,
                }),
            )
                .into_response()
        }
    }
}
