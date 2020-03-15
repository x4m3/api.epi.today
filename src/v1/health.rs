use actix_web::{get, http::StatusCode, web, HttpResponse, Responder};
use serde::Serialize;

#[path = "../intra_client.rs"]
mod intra_client;

#[get("/api")]
async fn api() -> impl Responder {
    HttpResponse::Ok().body("2epi2day4you\n")
}

#[derive(Serialize)]
struct IntraReply {
    msg: String,
}

#[get("/intra")]
async fn intra() -> impl Responder {
    let intra_reply: IntraReply;
    let client = intra_client::create_client().unwrap();

    let url = format!("https://intra.epitech.eu/?format=json");
    let res = client.get(&url).send().await.unwrap();

    if res.status() == StatusCode::FORBIDDEN {
        intra_reply = IntraReply {
            msg: String::from("okay"),
        }
    } else {
        intra_reply = IntraReply {
            msg: String::from("down"),
        }
    }

    web::Json(intra_reply)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(api);
    cfg.service(intra);
}
