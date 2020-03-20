use crate::intra::{autologin, client};
use crate::v1::data;
use actix_web::{get, http::StatusCode, HttpRequest, HttpResponse, Responder};
use serde_json::Value;

#[get("/info")]
pub async fn info(req: HttpRequest) -> impl Responder {
    let autologin = match autologin::get_from_header(&req) {
        Some(autologin) => autologin,
        _ => {
            return HttpResponse::BadRequest().json(data::Default {
                msg: String::from("no autologin provided"),
            })
        }
    };

    match autologin::check(&autologin) {
        Some(result) => {
            if result == false {
                return HttpResponse::BadRequest().json(data::Default {
                    msg: String::from("bad autologin provided"),
                });
            }
        }
        None => {
            return HttpResponse::InternalServerError().json(data::Default {
                msg: String::from("failed to check autologin"),
            })
        }
    }

    let client = match client::create_client() {
        Ok(client) => client,
        Err(_) => {
            return HttpResponse::InternalServerError().json(data::Default {
                msg: String::from("could not create intra client"),
            })
        }
    };

    let path = format!("/user/?format=json");
    let res = match client::get_path_auth(&client, &autologin, &path).await {
        Ok(res) => res,
        Err(_) => {
            return HttpResponse::ServiceUnavailable().json(data::Default {
                msg: String::from("error"),
            })
        }
    };

    if res.status() != StatusCode::OK {
        return HttpResponse::InternalServerError().json(data::Default {
            msg: String::from("could not get user information"),
        });
    }

    let raw_body = match res.text().await {
        Ok(raw_body) => raw_body,
        Err(_) => {
            return HttpResponse::InternalServerError().json(data::Default {
                msg: String::from("could not get intra response"),
            })
        }
    };

    let raw_json: Value = match serde_json::from_str(&raw_body) {
        Ok(raw_json) => raw_json,
        Err(_) => {
            return HttpResponse::InternalServerError().json(data::Default {
                msg: String::from("failed to parse intra response in json"),
            })
        }
    };

    let user = data::User {
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
