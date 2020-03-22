use actix_web::web;

mod event_register;
mod event_unregister;
mod rdv;
mod token;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(event_register::event_register);
    cfg.service(event_unregister::event_unregister);
    cfg.service(token::token);
    cfg.service(rdv::rdv);
}
