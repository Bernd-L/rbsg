mod routes;

use actix_web::{App, HttpServer};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting...");

    let future = HttpServer::new(move || App::new().configure(routes::config))
        .bind(env::var("RBSG_BIND").unwrap_or(String::from("0.0.0.0:8080")))?
        .run();

    println!("Started");

    // Don't exit the app; the server needs to keep running
    future.await
}
