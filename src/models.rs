use diesel::prelude::*;
use diesel::sql_types::Timestamp;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::links)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Link {
    pub url: String,
    pub slug: String,
    pub clicks: i32,
    created_at: Timestamp,
    updated_at: Timestamp,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::links)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewLink<'a> {
    pub url: &'a str,
    pub slug: &'a str,
}

pub fn get_link(conn: &mut PgConnection, slug: &str) -> QueryResult<Link> {
    use crate::schema::links::dsl::*;

    links.filter(slug.eq(slug))
        .first::<Link>(conn)
}

pub fn increment_clicks(conn: &mut PgConnection, slug: &str) -> QueryResult<Link> {
    use crate::schema::links::dsl::*;

    diesel::update(links.filter(slug.eq(slug)))
        .set(clicks.eq(clicks + 1))
        .get_result::<Link>(conn)
}