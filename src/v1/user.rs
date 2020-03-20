use crate::intra_autologin;
use crate::intra_client;
use actix_web::{get, http::StatusCode, web, HttpRequest, HttpResponse, Responder};
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
struct ReplyInfo {
    msg: String,
}

#[derive(Serialize)]
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

    let client = match intra_client::create_client() {
        Ok(client) => client,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ReplyInfo {
                msg: String::from("could not create intra client"),
            })
        }
    };

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

    let raw_body = match res.text().await {
        Ok(raw_body) => raw_body,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ReplyInfo {
                msg: String::from("could not get intra response"),
            })
        }
    };

    let raw_json: Value = match serde_json::from_str(&raw_body) {
        Ok(raw_json) => raw_json,
        Err(_) => {
            return HttpResponse::InternalServerError().json(ReplyInfo {
                msg: String::from("failed to parse intra response in json"),
            })
        }
    };

    let user = UserInfo {
        name: match raw_json["title"].as_str() {
            Some(name) => String::from(name),
            None => String::from("Ano Nymous"),
        },

        email: match raw_json["login"].as_str() {
            Some(email) => String::from(email),
            None => String::from("ano.nymous@epitech.eu"),
        },

        city: match raw_json["groups"][0]["title"].as_str() {
            Some(city) => String::from(city),
            None => String::from("Homeless"),
        },

        year: match raw_json["studentyear"].as_u64() {
            Some(year) => year,
            None => 42,
        },

        semester: match raw_json["semester"].as_u64() {
            Some(semester) => semester,
            None => 42,
        },

        credits: match raw_json["credits"].as_u64() {
            Some(credits) => credits,
            None => 0,
        },

        gpa: match raw_json["gpa"][0]["gpa"].as_str() {
            Some(gpa) => String::from(gpa),
            None => String::from("0.00"),
        },

        log: match raw_json["nsstat"]["active"].as_f64() {
            Some(log) => log,
            None => 0.00,
        },
    };

    HttpResponse::Ok().json(user)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(info);
}
