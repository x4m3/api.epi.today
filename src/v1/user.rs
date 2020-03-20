use crate::intra_autologin;
use crate::intra_client;
use actix_web::{get, http::StatusCode, web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct ReplyInfo {
    msg: String,
}

#[derive(Serialize, Deserialize)]
struct UserInfo {
    name: String,
    email: String,
    city: String,
    year: u8,
    semester: u8,
    credits: u16,
    gpa: f64,
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

    // TODO: deserialize res into structure
    // TODO: serialize into output

    match res.status() {
        StatusCode::OK => {
            return HttpResponse::Ok().json(ReplyInfo {
                msg: String::from("okay"),
            });
        }
        _ => {
            return HttpResponse::InternalServerError().json(ReplyInfo {
                msg: String::from("could not get user information"),
            });
        }
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(info);
}
