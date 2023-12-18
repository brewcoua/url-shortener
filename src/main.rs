#![deny(warnings)]

use std::convert::Infallible;
use std::net::SocketAddr;
use log::{info, error, trace};
use std_logger::request;

mod lib;

use dotenvy::dotenv;

use ntex::web;

#[web::get("/")]
async fn index() -> impl web::Responder {
    web::HttpResponse::Ok().body("Hello world!")
}

#[web::get("/{slug}")]
async fn redirect(conn: web::types::State<lib::AsyncPgConnection>,
                  path: web::types::Path<(String,)>) -> web::HttpResponse {
    // Find slug in database
    let link = match lib::models::get_link(&mut *conn, &path.0).await {
        Ok(link) => link,
        Err(e) => {
            error!("Error getting link: {}", e);
            return web::HttpResponse::InternalServerError().body("Internal Server Error");
        }
    };

    // If slug is found, redirect to URL
    if let Ok(Some(doc)) = coll.find_one(filter, None).await {
        let url = doc.get_str("url").unwrap();
        trace!("Redirecting to {}", url);
        return web::HttpResponse::Found()
            .header("Location", url)
            .finish();
    }

    // If slug is not found, return 404
    trace!("Slug not found");
    web::HttpResponse::NotFound().body("404 Not Found")
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std_logger::Config::json().init();

    let mut conn = lib::establish_connection().await;

    web::HttpServer::new(|| {
        web::App::new()
            .state(conn.clone())
            .service(index)
            .service(redirect)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}