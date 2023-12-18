#![deny(warnings)]

use log::{info, trace};
use dotenvy::dotenv;

use diesel::pg::PgConnection;
use diesel::r2d2::{Pool, ConnectionManager};

use ntex::web;

mod db;

type DbPool = Pool<ConnectionManager<PgConnection>>;

#[web::get("/")]
async fn index() -> impl web::Responder {
    web::HttpResponse::Found()
        .header("Location", "https://www.brewen.dev")
        .finish()
}

#[web::get("/{slug}")]
async fn redirect(pool: web::types::State<DbPool>,
                  path: web::types::Path<(String, )>) -> web::HttpResponse {
    let (slug, ) = path.into_inner();

    if slug.len() < 3 {
        trace!("GET /{} - 404 (too short)", slug);
        return web::HttpResponse::NotFound().finish();
    }

    let mut conn = pool.get().expect("Failed to get connection from pool");
    let result = web::block(move || {
        db::models::get_link(&mut conn, &slug)
    }).await
        .ok();

    let link = match result {
        None => {
            trace!("GET /{} - 404 (not found)", slug);
            return web::HttpResponse::NotFound().finish();
        }
        Some(link) => link
    };

    trace!("GET /{} - 302 -> {}", slug, link.url);
    web::HttpResponse::Found()
        .header("Location", link.url)
        .finish()
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std_logger::Config::json().init();

    let manager = ConnectionManager::<PgConnection>::new(
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set")
    );
    let pool = Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Failed to create pool");
    info!("Created database pool");

    web::HttpServer::new(move || {
        web::App::new()
            .state(pool.clone())
            .service(index)
            .service(redirect)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}