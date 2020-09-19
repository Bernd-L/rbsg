use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::{env, sync::Mutex};

pub fn config(cfg: &mut web::ServiceConfig) {
  cfg.service(web::scope("/v1"));
}
