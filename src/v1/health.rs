use actix_web::{get, http::StatusCode, web, Responder};
use serde::Serialize;

#[path = "../intra_client.rs"]
mod intra_client;

#[derive(Serialize)]
struct ReplyInfo {
    msg: String,
}

#[get("/api")]
async fn api() -> impl Responder {
    web::Json(ReplyInfo {
        msg: String::from("2epi2day4you"),
    })
}

#[get("/intra")]
async fn intra() -> impl Responder {
    let client = intra_client::create_client().unwrap();

    let url = format!("https://intra.epitech.eu/?format=json");
    let res = match intra_client::make_get_request(&client, &url).await {
        Ok(res) => res,
        Err(_) => {
            return web::Json(ReplyInfo {
                msg: String::from("error"),
            })
        }
    };

    match res.status() {
        StatusCode::FORBIDDEN => {
            return web::Json(ReplyInfo {
                msg: String::from("okay"),
            })
        }
        _ => {
            return web::Json(ReplyInfo {
                msg: String::from("down"),
            })
        }
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(api);
    cfg.service(intra);
}
