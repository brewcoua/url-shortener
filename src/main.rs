#![deny(warnings)]

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tower::ServiceBuilder;
use sentry::integrations::tower::{NewSentryLayer, SentryHttpLayer};

mod db;
mod routes;
mod state;

#[tokio::main]
async fn main() {
    dotenv().ok();
    state::setup_tracing();
    state::setup_sentry();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&*std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to create pool");
    tracing::info!("Connected to database and created pool");

    let app_state = state::AppState {
        pool
    };

    let layer = ServiceBuilder::new()
        .layer(NewSentryLayer::new_from_top())
        .layer(SentryHttpLayer::with_transaction());

    let app = routes::declare(app_state)
        .layer(layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}



