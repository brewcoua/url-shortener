use sqlx::Error;
use crate::state::{DbPool};

#[derive(sqlx::FromRow)]
pub struct Link {
    pub slug: String,
    pub url: String,
    pub clicks: i32,
}

#[tracing::instrument(skip(conn))]
pub async fn get_link_and_increment(conn: &DbPool, slugs: &str) -> Result<Link, Error> {
    sqlx::query_as::<_, Link>(
        r#"
        UPDATE links
        SET clicks = clicks + 1
        WHERE slug = $1
        RETURNING *
        "#,
    )
        .bind(slugs)
        .fetch_one(conn)
        .await
}
