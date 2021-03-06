use crate::v1::data;
use actix_web::{get, HttpResponse, Responder};

#[get("/api")]
pub async fn api() -> impl Responder {
    HttpResponse::Ok().json(data::Default {
        msg: String::from(env!("CARGO_PKG_VERSION")),
    })
}
