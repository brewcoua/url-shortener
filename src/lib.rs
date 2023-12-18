use diesel_async::{ RunQueryDsl, AsyncConnection, AsyncPgConnection };
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub async fn establish_connection() -> AsyncPgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    AsyncPgConnection::establish(&database_url?)
        .await?
}