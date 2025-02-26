use axum::Router;
use axum::routing::{get, post};
use tower_http::cors::{CorsLayer, Any};
use crate::{AppState};
use crate::api::handler;

pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        // `GET /` goes to `root`
        .route("/", get(handler::index))
        .route("/authorize", post(handler::authorize))
        .route("/me", get(handler::decode_cookie))
        .with_state(state)
        .layer(cors)
}