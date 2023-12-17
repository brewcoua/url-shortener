#![deny(warnings)]

use std::convert::Infallible;
use std::net::SocketAddr;
use log::{info, error, trace};
use std_logger::request;

use dotenv::dotenv;

use mongodb::{Client, options::ClientOptions};
use mongodb::bson::{doc, Document};

use bytes::Bytes;
use http_body_util::Full;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

static mut CLIENT: Option<Client> = None;

async fn handle(req: Request<impl hyper::body::Body>) -> Result<Response<Full<Bytes>>, Infallible> {
    request!("GET {}", req.uri());

    // Get slug from URL
    let slug = req.uri().path().trim_start_matches('/');

    // Find slug in database
    let client = unsafe { CLIENT.as_ref().unwrap() };
    let db = client.database("prod");
    let coll = db.collection::<Document>("links");
    let filter = doc! { "slug": slug };

    // If slug is found, redirect to URL
    if let Ok(Some(doc)) = coll.find_one(filter, None).await {
        let url = doc.get_str("url").unwrap();
        trace!("Redirecting to {}", url);
        return Ok(Response::builder()
            .status(301)
            .header("Location", url)
            .body(Full::new(Bytes::new()))
            .unwrap());
    }

    // If slug is not found, return 404
    trace!("Slug not found");
    Ok(Response::builder()
        .status(404)
        .body(Full::new(Bytes::new()))
        .unwrap())
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();
    std_logger::Config::json().init();

    let db_cluster: String = std::env::var("DB_CLUSTER").expect("DB_CLUSTER must be set");
    let db_user: String = std::env::var("DB_USER").expect("DB_USER must be set");
    let db_pass: String = std::env::var("DB_PASS").expect("DB_PASSWORD must be set");
    let db_url: String = "mongodb+srv://".to_owned() + &db_user + ":" + &db_pass + "@" + &db_cluster + "/test?retryWrites=true&w=majority";

    info!("Connecting to MongoDB at {}", db_cluster);
    let client_options = ClientOptions::parse(db_url)
        .await?;


    unsafe {
        CLIENT = Some(Client::with_options(client_options)?);
    }
    info!("Connected to MongoDB");

    let addr = SocketAddr::from(
        ([0, 0, 0, 0, 0, 0, 0, 0], std::env::var("PORT").unwrap_or("8080".to_string()).parse().unwrap())
    );

    // Bind to the port and listen for incoming TCP connections
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on http://{}", addr);
    loop {
        let (tcp, _) = listener.accept().await?;
        let io = TokioIo::new(tcp);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(handle))
                .await
            {
                error!("Error while serving HTTP connection: {:?}", err);
            }
        });
    }
}