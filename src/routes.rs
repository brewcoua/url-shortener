use axum::Router;
use axum::routing::get;
use crate::state::AppState;

mod redirect;
mod root;
mod health;

pub fn declare(state: AppState) -> Router {
    Router::new()
        .route("/", get(root::root))
        .route("/health/check", get(health::health))
        .route("/:slug", get(redirect::redirect))
        .with_state(state)
}