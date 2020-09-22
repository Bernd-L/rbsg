use actix_web::{get, http::StatusCode, web, HttpResponse, Responder};

pub const INDEX_HTML: &'static str = include_str!("../../../web/index.html");
pub const RBSG_BG_WASM: &[u8] = include_bytes!("../../../target/static/rbsg_bg.wasm");
pub const RBSG_JS: &'static str = include_str!("../../../target/static/rbsg.js");

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg
    .service(index_html)
    .service(rbsg_bg_wasm)
    .service(rbsg_js);
}

#[get("/index.html")]
async fn index_html() -> impl Responder {
  HttpResponse::Ok().body(INDEX_HTML)
}

#[get("/rbsg_bg.wasm")]
async fn rbsg_bg_wasm() -> impl Responder {
  HttpResponse::build(StatusCode::OK)
    .content_type("application/wasm")
    .body(RBSG_BG_WASM)
}

#[get("/rbsg.js")]
async fn rbsg_js() -> impl Responder {
  HttpResponse::build(StatusCode::OK)
    .content_type("text/javascript")
    .body(RBSG_JS)
}
