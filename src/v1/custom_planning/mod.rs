use actix_web::web;

mod day;
mod list;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list::list);
    cfg.service(day::day);
}
