use actix_web::{get, post, web, HttpResponse, Responder};
use std::sync::Mutex;

pub fn config(cfg: &mut web::ServiceConfig) {
  let demo_data = web::Data::new(CountState {
    count: Mutex::new(0),
  });

  cfg.service(
    web::scope("/")
      .app_data(demo_data.clone())
      .route("/hey", web::get().to(manual_hello))
      .service(stateful_counter)
      .service(echo)
      .service(hello),
  );
}

#[get("/hw")]
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

struct CountState {
  count: Mutex<usize>,
}
