#![deny(warnings)]

use dotenvy::dotenv;
use diesel::pg::PgConnection;
use diesel::r2d2::{Pool, ConnectionManager};

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

    let manager = ConnectionManager::<PgConnection>::new(
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set")
    );
    let pool = Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Failed to create pool");
    tracing::info!("Created database pool");

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



