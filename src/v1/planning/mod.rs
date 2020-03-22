use actix_web::web;

mod event_register;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(event_register::event_register);
}
