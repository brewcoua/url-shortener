use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};

use crate::{
    state::{AppState},
    db::models
};

#[tracing::instrument(skip(state))]
pub async fn redirect(
    State(state): State<AppState>,
    Path(slug): Path<String>
) -> impl IntoResponse {
    if slug.len() < 3 {
        tracing::info!("Slug too small (<3)");
        return StatusCode::NOT_FOUND.into_response();
    }

    let link = models::get_link_and_increment(&state.pool, &slug).await;
    match link {
        Ok(link) => {
            tracing::info!("Slug found and redirected");
            Redirect::temporary(&link.url).into_response()
        }
        Err(err) => {
            if let sqlx::Error::RowNotFound = err {
                tracing::info!("Slug not found");
                return StatusCode::NOT_FOUND.into_response();
            }
            tracing::info!("Unknown error: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}