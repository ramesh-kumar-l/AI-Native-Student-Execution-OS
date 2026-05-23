/// Axum route handlers for the HTTP loopback IPC server.
/// Surfaces: Desktop app + VSCode extension both speak this API.
use axum::{
    extract::{Path, Query, State},
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
    engines::mentor::MentorTurnInput,
    ipc::types::{
        ApiError, CreateProjectRequest, CreateTaskRequest, HealthResponse, LimitQuery,
        MentorTurnChunk, MentorTurnRequest, UpdateProjectRequest, UpdateTaskRequest,
    },
    AppState,
};

// ── Helpers ───────────────────────────────────────────────────────────────────

fn not_found(msg: &str) -> (StatusCode, Json<ApiError>) {
    (
        StatusCode::NOT_FOUND,
        Json(ApiError {
            error: msg.to_string(),
            correlation_id: None,
        }),
    )
}

fn internal_err(e: impl std::fmt::Display) -> (StatusCode, Json<ApiError>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ApiError {
            error: e.to_string(),
            correlation_id: None,
        }),
    )
}

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

/// GET /api/v1/audit/recent?limit=N
pub async fn audit_recent(
    State(state): State<AppState>,
    Query(q): Query<LimitQuery>,
) -> impl IntoResponse {
    match state.audit.recent(q.limit).await {
        Ok(entries) => Json(entries).into_response(),
        Err(e) => {
            error!(err = %e, "audit recent query failed");
            internal_err(e).into_response()
        }
    }
}

// ── Projects ──────────────────────────────────────────────────────────────────

/// GET /api/v1/projects
pub async fn list_projects(State(state): State<AppState>) -> impl IntoResponse {
    match state.memory.l1.list_projects().await {
        Ok(projects) => Json(projects).into_response(),
        Err(e) => {
            error!(err = %e, "list_projects failed");
            internal_err(e).into_response()
        }
    }
}

/// POST /api/v1/projects
pub async fn create_project(
    State(state): State<AppState>,
    Json(req): Json<CreateProjectRequest>,
) -> impl IntoResponse {
    match state.memory.l1.create_project(req.name, req.description).await {
        Ok(p) => (StatusCode::CREATED, Json(p)).into_response(),
        Err(e) => {
            error!(err = %e, "create_project failed");
            internal_err(e).into_response()
        }
    }
}

/// GET /api/v1/projects/:id
pub async fn get_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match state.memory.l1.get_project(&id).await {
        Ok(Some(p)) => Json(p).into_response(),
        Ok(None) => not_found("project not found").into_response(),
        Err(e) => {
            error!(err = %e, id, "get_project failed");
            internal_err(e).into_response()
        }
    }
}

/// PUT /api/v1/projects/:id
pub async fn update_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateProjectRequest>,
) -> impl IntoResponse {
    let existing = match state.memory.l1.get_project(&id).await {
        Ok(Some(p)) => p,
        Ok(None) => return not_found("project not found").into_response(),
        Err(e) => return internal_err(e).into_response(),
    };

    let name = req.name.unwrap_or(existing.name);
    let description = req.description.or(existing.description);
    let status = req.status.unwrap_or(existing.status);

    match state.memory.l1.update_project(&id, name, description, status).await {
        Ok(Some(p)) => Json(p).into_response(),
        Ok(None) => not_found("project not found").into_response(),
        Err(e) => {
            error!(err = %e, id, "update_project failed");
            internal_err(e).into_response()
        }
    }
}

/// DELETE /api/v1/projects/:id
pub async fn delete_project(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match state.memory.l1.delete_project(&id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            error!(err = %e, id, "delete_project failed");
            internal_err(e).into_response()
        }
    }
}

// ── Tasks ─────────────────────────────────────────────────────────────────────

/// GET /api/v1/projects/:id/tasks
pub async fn list_tasks(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
) -> impl IntoResponse {
    match state.memory.l1.list_tasks_for_project(&project_id).await {
        Ok(tasks) => Json(tasks).into_response(),
        Err(e) => {
            error!(err = %e, project_id, "list_tasks failed");
            internal_err(e).into_response()
        }
    }
}

/// POST /api/v1/projects/:id/tasks
pub async fn create_task(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
    Json(req): Json<CreateTaskRequest>,
) -> impl IntoResponse {
    match state
        .memory
        .l1
        .create_task(project_id, req.title, req.description, req.priority, req.due_at)
        .await
    {
        Ok(t) => (StatusCode::CREATED, Json(t)).into_response(),
        Err(e) => {
            error!(err = %e, "create_task failed");
            internal_err(e).into_response()
        }
    }
}

/// PUT /api/v1/tasks/:id
pub async fn update_task(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateTaskRequest>,
) -> impl IntoResponse {
    let existing = match state.memory.l1.get_task(&id).await {
        Ok(Some(t)) => t,
        Ok(None) => return not_found("task not found").into_response(),
        Err(e) => return internal_err(e).into_response(),
    };

    let title = req.title.unwrap_or(existing.title);
    let description = req.description.or(existing.description);
    let status = req.status.unwrap_or(existing.status);
    let priority = req.priority.unwrap_or(existing.priority);
    let due_at = if req.due_at.is_some() { req.due_at } else { existing.due_at };

    match state.memory.l1.update_task(&id, title, description, status, priority, due_at).await {
        Ok(Some(t)) => Json(t).into_response(),
        Ok(None) => not_found("task not found").into_response(),
        Err(e) => {
            error!(err = %e, id, "update_task failed");
            internal_err(e).into_response()
        }
    }
}

/// DELETE /api/v1/tasks/:id
pub async fn delete_task(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match state.memory.l1.delete_task(&id).await {
        Ok(()) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            error!(err = %e, id, "delete_task failed");
            internal_err(e).into_response()
        }
    }
}

// ── Conversations ─────────────────────────────────────────────────────────────

/// GET /api/v1/conversations?limit=N
pub async fn list_conversations(
    State(state): State<AppState>,
    Query(q): Query<LimitQuery>,
) -> impl IntoResponse {
    match state.memory.l1.list_conversations(q.limit).await {
        Ok(convs) => Json(convs).into_response(),
        Err(e) => {
            error!(err = %e, "list_conversations failed");
            internal_err(e).into_response()
        }
    }
}

/// GET /api/v1/conversations/:id/messages?limit=N
pub async fn get_conversation_messages(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(q): Query<LimitQuery>,
) -> impl IntoResponse {
    match state.memory.l1.messages_for_conversation(&id, q.limit).await {
        Ok(msgs) => Json(msgs).into_response(),
        Err(e) => {
            error!(err = %e, id, "get_conversation_messages failed");
            internal_err(e).into_response()
        }
    }
}
