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
        // ── Phase 3 · VSCode workflow signals ─────────────────────────────────
        .route("/api/v1/ws", get(routes::ws_signals))
        .route("/api/v1/signals", get(routes::list_signals))
        // ── Phase 4 · Capability engine ────────────────────────────────────────
        .route("/api/v1/capability/scores", get(routes::get_capability_scores))
        .route("/api/v1/capability/compute", post(routes::compute_capability))
        .route("/api/v1/capability/narrative", get(routes::get_capability_narrative))
        // ── Phase 4 · Artifact engine ──────────────────────────────────────────
        // Note: /artifacts/generate must be registered before /artifacts/:id so
        // the static path wins over the parameterized one.
        .route("/api/v1/artifacts", get(routes::list_artifacts))
        .route("/api/v1/artifacts/generate", post(routes::generate_artifact))
        .route(
            "/api/v1/artifacts/:id",
            get(routes::get_artifact).delete(routes::delete_artifact),
        )
        // ── Phase 5 · Trust engine ─────────────────────────────────────────────
        .route("/api/v1/trust/health", get(routes::trust_health))
        .route("/api/v1/trust/lineage", get(routes::trust_lineage))
        // ── Phase 5 · Sync coordinator ─────────────────────────────────────────
        .route("/api/v1/sync/status", get(routes::sync_status))
        .route("/api/v1/sync/export", post(routes::sync_export))
        .route("/api/v1/sync/import", post(routes::sync_import))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
