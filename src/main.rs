use crate::api::router::create_router;
use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;

mod api;
mod workflow;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = AppState {
        key: Key::generate(),
    };

    // build our application with a route
    let app = create_router(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("localhost:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// our application state
#[derive(Clone)]
struct AppState {
    // that holds the key used to sign cookies
    key: Key,
}

// this impl tells `SignedCookieJar` how to access the key from our state
impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}
