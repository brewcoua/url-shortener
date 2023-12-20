use tokio::task;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};

use crate::{
    state::{AppState},
    db::models
};

pub async fn redirect(
    State(state): State<AppState>,
    Path(slug): Path<String>
) -> impl IntoResponse {
    let span = tracing::span!(tracing::Level::INFO, "Redirect");
    let _guard = span.enter();

    if slug.len() < 3 {
        tracing::info!("Slug too small (<3)");
        return StatusCode::NOT_FOUND.into_response();
    }

    let cl_slug = slug.clone();

    let mut conn = state.pool.get().expect("Failed to get connection from pool");
    let result = task::spawn_blocking(move || {
        let link = models::get_link(&mut conn, &cl_slug);
        match link {
            Ok(link) => {
                models::increment_clicks(&mut conn, &cl_slug).expect("Failed to increment clicks");
                Ok(link)
            },
            Err(e) => Err(e)
        }
    }).await.expect("Failed to run blocking task");


    // Check if the slug exists
    let link = match result {
        Ok(link) => link,
        Err(e) => {
            if e == diesel::NotFound {
                tracing::info!("Slug not found");
                return StatusCode::NOT_FOUND.into_response();
            }
            tracing::error!("Failed to get link: {:?}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    tracing::info!("Slug found and redirected");
    Redirect::temporary(&link.url).into_response()
}