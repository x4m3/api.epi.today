use actix_web::web;

mod day;
mod event_register;
mod list;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list::list);
    cfg.service(day::day);
    cfg.service(event_register::event_register);
}
