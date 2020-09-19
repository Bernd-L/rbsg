mod api;
mod demo;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg
    .service(web::scope("/demo").configure(demo::config))
    .service(web::scope("/api").configure(api::config));
}
