use actix_web::web;

mod info;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(info::info);
}
