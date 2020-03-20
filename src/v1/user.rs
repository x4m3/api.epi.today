use crate::intra_autologin;
use crate::intra_client;
use actix_web::{get, http::StatusCode, web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize)]
struct ReplyInfo {
    msg: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct UserInfo {
    name: String,
    email: String,
    city: String,
    year: u64,
    semester: u64,
    credits: u64,
    gpa: String,
    log: f64,
}

#[get("/info")]
async fn info(req: HttpRequest) -> impl Responder {
    let autologin = match intra_autologin::get_from_header(&req) {
        Some(autologin) => autologin,
        _ => {
            return HttpResponse::BadRequest().json(ReplyInfo {
                msg: String::from("no autologin provided"),
            })
        }
    };

    if intra_autologin::check(&autologin) == false {
        return HttpResponse::BadRequest().json(ReplyInfo {
            msg: String::from("bad autologin provided"),
        });
    }

    let client = intra_client::create_client().unwrap();

    let path = format!("/user/?format=json");
    let res = match intra_client::get_path_auth(&client, &autologin, &path).await {
        Ok(res) => res,
        Err(_) => {
            return HttpResponse::ServiceUnavailable().json(ReplyInfo {
                msg: String::from("error"),
            })
        }
    };

    if res.status() != StatusCode::OK {
        return HttpResponse::InternalServerError().json(ReplyInfo {
            msg: String::from("could not get user information"),
        });
    }

    let raw_body = res.text().await.unwrap();
    let raw_json: Value = serde_json::from_str(&raw_body).unwrap();

    let user = UserInfo {
        name: String::from(raw_json["title"].as_str().unwrap()),
        email: String::from(raw_json["internal_email"].as_str().unwrap()),
        city: String::from(raw_json["groups"][0]["title"].as_str().unwrap()),
        year: raw_json["studentyear"].as_u64().unwrap(),
        semester: raw_json["semester"].as_u64().unwrap(),
        credits: raw_json["credits"].as_u64().unwrap(),
        gpa: String::from(raw_json["gpa"][0]["gpa"].as_str().unwrap()),
        log: raw_json["nsstat"]["active"].as_f64().unwrap(),
    };

    HttpResponse::Ok().json(user)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(info);
}
