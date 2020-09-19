use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::{env, sync::Mutex};

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

#[get("/count")]
async fn stateful_counter(data: web::Data<CountState>) -> impl Responder {
    // Get the count from the Mutex
    let mut cnt = data.count.lock().unwrap();

    // Increment
    *cnt += 1;

    // Send a response
    HttpResponse::Ok().body(format!("Count at: {}", cnt))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting...");

    let demo_data = web::Data::new(CountState {
        count: Mutex::new(0),
    });

    let future = HttpServer::new(move || {
        App::new()
            // .service(web::resource("/{project_id}").route(web::get().to(|| HttpResponse::Ok())))
            .service(
                web::scope("/demo")
                    .app_data(demo_data.clone())
                    .route("/hey", web::get().to(manual_hello))
                    .service(stateful_counter)
                    .service(echo)
                    .service(hello),
            )
            .service(web::scope("/api").service(web::scope("/v1")))
    })
    .bind(env::var("RBSG_BIND").unwrap_or(String::from("0.0.0.0:8080")))?
    .run();

    println!("Started");

    // Don't exit the app; the server needs to keep running
    future.await
}

struct CountState {
    count: Mutex<usize>,
}
