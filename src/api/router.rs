use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, ORIGIN};
use axum::Router;
use axum::routing::{get, post};
use tower_http::cors::{CorsLayer, Any, AllowHeaders, AllowMethods, AllowOrigin};
use crate::{AppState};
use crate::api::handler;

pub fn create_router(state: AppState) -> Router {

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list([
            "http://localhost:3000".parse().unwrap(),
            "http://localhost:5173".parse().unwrap(),
        ]))
        .allow_methods(AllowMethods::list([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::OPTIONS,
        ]))
        .allow_headers(AllowHeaders::list([
            CONTENT_TYPE,
            AUTHORIZATION,
            ACCEPT,
            ORIGIN,
        ]))
        .allow_credentials(true);

    Router::new()
        // `GET /` goes to `root`
        .route("/", get(handler::index))
        .route("/authorize", post(handler::authorize))
        .route("/me", get(handler::decode_cookie))
        .with_state(state)
        .layer(cors)
}