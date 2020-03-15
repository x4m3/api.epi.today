use actix_web::{get, post, web, HttpResponse, Responder};

#[post("/post/test")]
async fn post_test() -> impl Responder {
    HttpResponse::Ok().body("/post/test")
}

#[get("/post/test2")]
async fn post_test2() -> impl Responder {
    HttpResponse::Ok().body("/post/test2")
}

#[get("/get/test")]
async fn get_test() -> impl Responder {
    HttpResponse::Ok().body("/get/test")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_test);
    cfg.service(post_test);
    cfg.service(post_test2);
}
