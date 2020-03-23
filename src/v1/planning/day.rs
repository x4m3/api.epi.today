use crate::intra::{autologin, check, client};
use crate::v1::data;
use actix_web::{get, http::StatusCode, web, HttpRequest, HttpResponse, Responder};
use serde_json::Value;

#[get("/day")]
pub async fn day(req: HttpRequest, input: web::Json<data::PlanningDayInput>) -> impl Responder {
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

    let full_date = match check::yyyy_mm_dd(&input.date) {
        Some(full_date) => full_date,
        None => {
            return HttpResponse::BadRequest().json(data::Default {
                msg: String::from("invalid date provided"),
            });
        }
    };

    let client = match client::create_client() {
        Ok(client) => client,
        Err(_) => {
            return HttpResponse::InternalServerError().json(data::Default {
                msg: String::from("could not create intra client"),
            })
        }
    };

    let formatted_date = full_date.format("%Y-%m-%d").to_string();
    let path = format!(
        "/planning/load?format=json&start={}&end={}",
        formatted_date, formatted_date
    );
    let res = match client::get_path_auth(&client, &autologin, &path).await {
        Ok(res) => res,
        Err(_) => {
            return HttpResponse::ServiceUnavailable().json(data::Default {
                msg: String::from("client error"),
            })
        }
    };

    if res.status() != StatusCode::OK {
        return HttpResponse::InternalServerError().json(data::Default {
            msg: String::from("could not get custom_planning information"),
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

    let mut list: Vec<data::PlanningDayResult> = Vec::new();

    // if json parsing fails, that means there are no events
    // json parsing fails because the intra returns an empty object
    // and we are expecting a vector
    let raw_json: Vec<Value> = match serde_json::from_str(&raw_body) {
        Ok(raw_json) => raw_json,
        Err(_) => return HttpResponse::Ok().json(list),
    };

    println!("{:?}", raw_json);

    for event in &raw_json {}

    HttpResponse::Ok().json(list)
}
