use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use actix_web_prom::PrometheusMetrics;
use dotenv::dotenv;
use std::env;

#[macro_use]
extern crate log;

mod intra;
mod v1;

#[get("/")]
async fn root_doc() -> impl Responder {
    HttpResponse::Ok()
        // set as utf8 html file
        .content_type("text/html; charset=utf-8")
        // and no need to have the html file at runtime
        .body(include_str!("../doc/doc.html"))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Starting logger
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("hello world");

    // Reading env file
    dotenv().ok().expect("Failed to read .env file");
    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT").expect("Port not set");

    // Stats
    let prometheus = PrometheusMetrics::new("api", Some("/stats"), None);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::new("[HTTP %s] [URL %U]"))
            .wrap(prometheus.clone())
            .service(root_doc)
            .service(web::scope("/v1").configure(v1::init_routes))
    });

    info!(
        "starting server on http:://{}",
        format!("{}:{}", host, port)
    );

    server.bind(format!("{}:{}", host, port))?.run().await
}
