use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::env;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn stateful_counter() -> impl Responder {
    HttpResponse::Ok().body("0")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting...");

    let mut stateful_count = 0;

    let future = HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            // .service(web::resource("/{project_id}").route(web::get().to(|| HttpResponse::Ok())))
            .route("/hey", web::get().to(manual_hello))
            .route("/count", web::get().to(stateful_counter))
    })
    .bind(env::var("RBSG_BIND").unwrap_or(String::from("0.0.0.0:8080")))?
    .run();

    println!("Started");

    // Don't exit the app; the server needs to keep running
    future.await
}
