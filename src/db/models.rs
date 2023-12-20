use diesel::data_types::PgTimestamp;
use diesel::prelude::*;
use crate::state::{DbConn};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::links)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Link {
    pub url: String,
    pub slug: String,
    pub clicks: i32,
    pub created_at: PgTimestamp,
    pub updated_at: PgTimestamp,
}

#[tracing::instrument(skip(conn))]
pub fn get_link(conn: &mut DbConn, slugs: &str) -> QueryResult<Link> {
    use crate::db::schema::links::dsl::*;

    links
        .filter(slug.eq(slugs))
        .first::<Link>(conn)
}

#[tracing::instrument(skip(conn))]
pub fn increment_clicks(conn: &mut DbConn, slugs: &str) -> QueryResult<usize> {
    use crate::db::schema::links::dsl::*;

    diesel::update(links.filter(slug.eq(slugs)))
        .set(clicks.eq(clicks + 1))
        .execute(conn)
}
