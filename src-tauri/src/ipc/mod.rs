pub mod routes;
pub mod types;

use axum::{
    routing::{get, post},
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
        .route("/api/v1/health", get(routes::health))
        .route("/api/v1/mentor/turn", post(routes::mentor_turn))
        .route("/api/v1/audit/recent", get(routes::audit_recent))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
