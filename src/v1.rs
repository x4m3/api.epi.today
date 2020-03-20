use actix_web::web;

// Data types
mod data;

// Routes
mod custom_planning;
mod health;
mod planning;
mod user;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/health").configure(health::init_routes));
    cfg.service(web::scope("/user").configure(user::init_routes));
    cfg.service(web::scope("/planning").configure(planning::init_routes));
    cfg.service(web::scope("/custom_planning").configure(custom_planning::init_routes));
}
