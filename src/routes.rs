use axum::Router;
use axum::routing::get;
use crate::state::AppState;

pub mod redirect;
pub mod root;

pub fn declare(state: AppState) -> Router {
    Router::new()
        .route("/", get(root::root))
        .route("/:slug", get(redirect::redirect))
        .with_state(state)
}