use actix_web::web;

mod api;
mod intra;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(api::api);
    cfg.service(intra::intra);
}
