#![deny(warnings)]

use dotenvy::dotenv;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, filter::EnvFilter, Registry};

use diesel::pg::PgConnection;
use diesel::r2d2::{Pool, ConnectionManager};

use axum::{
    Router,
    routing::get,
};

mod db;
mod routes;
mod state;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new(
        "axum".into(),
        std::io::stdout
    );
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    set_global_default(subscriber).expect("Failed to set subscriber");


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

    let app = Router::new()
        .route("/", get(routes::root::root))
        .route("/:slug", get(routes::redirect::redirect))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}



