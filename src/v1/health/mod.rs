use actix_web::{get, web, HttpResponse, Responder};

#[get("/api")]
async fn api() -> impl Responder {
    HttpResponse::Ok().body("2epi2day4you\n")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(api);
}
