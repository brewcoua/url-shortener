use diesel::prelude::*;
use crate::state::{DbConn};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::links)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Link {
    pub url: String,
    pub slug: String,
}

pub fn get_link(conn: &mut DbConn, slugs: &str) -> QueryResult<Link> {
    use crate::db::schema::links::dsl::*;

    links
        .filter(slug.eq(slugs))
        .first::<Link>(conn)
}
