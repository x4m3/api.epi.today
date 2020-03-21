use crate::intra::{autologin, client};
use crate::v1::data;
use actix_web::{get, http::StatusCode, web, HttpRequest, HttpResponse, Responder};
use chrono::NaiveDateTime;
use serde_json::Value;

#[get("/day")]
pub async fn day(
    req: HttpRequest,
    input: web::Json<data::CustomPlanningEventInput>,
) -> impl Responder {
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

    let full_date = match NaiveDateTime::parse_from_str(
        &format!("{} 00:00:00", input.date),
        "%Y-%m-%d %H:%M:%S",
    ) {
        Ok(full_date) => full_date,
        Err(_) => {
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

    let date = full_date.format("%Y-%m-%d").to_string();
    let path = format!(
        "/planning/{}/events?format=json&start={}&end={}",
        input.calendar_id, date, date
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

    let mut list: Vec<data::CustomPlanningEventResult> = Vec::new();

    // if json parsing fails, that means there are no plannings
    // json parsing fails because the intra returns an empty object
    // and we are expecting a vector
    let raw_json: Vec<Value> = match serde_json::from_str(&raw_body) {
        Ok(raw_json) => raw_json,
        Err(_) => return HttpResponse::Ok().json(list),
    };

    // for planning in &raw_json {
    //     list.push(data::CustomPlanningEventResult {
    //         id: match planning["id"].as_u64() {
    //             Some(id) => id,
    //             None => {
    //                 return HttpResponse::InternalServerError().json(data::Default {
    //                     msg: String::from("value `id` does not exist"),
    //                 })
    //             }
    //         },

    //         name: match planning["title"].as_str() {
    //             Some(name) => String::from(name),
    //             None => {
    //                 return HttpResponse::InternalServerError().json(data::Default {
    //                     msg: String::from("value `title` does not exist"),
    //                 })
    //             }
    //         },
    //     })
    // }

    HttpResponse::Ok().json(list)
}
