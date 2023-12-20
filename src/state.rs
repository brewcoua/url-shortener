use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, filter::EnvFilter, Registry};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConn = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
}

pub fn setup_tracing() {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new(
        "axum".into(),
        std::io::stdout,
    );
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .with(sentry::integrations::tracing::layer());
    set_global_default(subscriber).expect("Failed to set subscriber");
}

pub fn setup_sentry() {
    let _guard = sentry::init((
        std::env::var("SENTRY_DSN").expect("SENTRY_DSN must be set"), sentry::ClientOptions {
            release: sentry::release_name!(),
            traces_sample_rate: 1.0,
            ..Default::default()
        }));
}