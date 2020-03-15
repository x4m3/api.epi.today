use actix_web::web;

mod health;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/health").configure(health::init_routes));
}
