pub mod routes;
pub mod types;

use axum::{
    routing::{get, post, put},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::AppState;

pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        // ── System ──────────────────────────────────────────────────────────
        .route("/api/v1/health", get(routes::health))
        // ── Mentor ──────────────────────────────────────────────────────────
        .route("/api/v1/mentor/turn", post(routes::mentor_turn))
        // ── Conversations ────────────────────────────────────────────────────
        .route("/api/v1/conversations", get(routes::list_conversations))
        .route(
            "/api/v1/conversations/:id/messages",
            get(routes::get_conversation_messages),
        )
        // ── Projects ─────────────────────────────────────────────────────────
        .route("/api/v1/projects", get(routes::list_projects).post(routes::create_project))
        .route(
            "/api/v1/projects/:id",
            get(routes::get_project)
                .put(routes::update_project)
                .delete(routes::delete_project),
        )
        // ── Tasks ─────────────────────────────────────────────────────────────
        .route(
            "/api/v1/projects/:id/tasks",
            get(routes::list_tasks).post(routes::create_task),
        )
        .route(
            "/api/v1/tasks/:id",
            put(routes::update_task).delete(routes::delete_task),
        )
        // ── Audit ─────────────────────────────────────────────────────────────
        .route("/api/v1/audit/recent", get(routes::audit_recent))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
