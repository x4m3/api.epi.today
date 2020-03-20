use crate::intra_client;
use actix_web::{get, http::StatusCode, web, HttpResponse, Responder};
use serde::Serialize;

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
    let client = match intra_client::create_client() {
        Ok(client) => client,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ReplyInfo {
                msg: String::from("could not create intra client"),
            })
        }
    };

    let path = format!("/?format=json");
    let res = match intra_client::get_path(&client, &path).await {
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
            // otherwise, the intra is (probably) down
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
