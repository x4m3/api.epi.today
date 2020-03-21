use actix_web::web;

mod list;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list::list);
}
