use actix_web::{get, http::StatusCode, web, HttpResponse, Responder};
use serde::Serialize;

use crate::intra_client;

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
        // if request fails, it may be an error from our end or something else
        Err(_) => {
            return HttpResponse::ServiceUnavailable().json(ReplyInfo {
                msg: String::from("error"),
            })
        }
    };

    match res.status() {
        StatusCode::FORBIDDEN => {
            // if intra return 403, that means that intra works
            // (403 because we don't have permission to get data)
            return HttpResponse::Ok().json(ReplyInfo {
                msg: String::from("okay"),
            });
        }
        _ => {
            // otherwise, the intra is (probably down)
            return HttpResponse::InternalServerError().json(ReplyInfo {
                msg: String::from("down"),
            });
        }
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(api);
    cfg.service(intra);
}
