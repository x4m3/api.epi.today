use crate::intra::{autologin, client};
use crate::v1::data;
use actix_web::{get, http::StatusCode, web, HttpRequest, HttpResponse, Responder};
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

    // TODO: get json values and check year, month, day (check if valid)

    let client = match client::create_client() {
        Ok(client) => client,
        Err(_) => {
            return HttpResponse::InternalServerError().json(data::Default {
                msg: String::from("could not create intra client"),
            })
        }
    };

    let path = format!("/planning/xyz/events?format=json&start=yyyy-mm-dd&end=yyyy-mm-dd");
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

    for planning in &raw_json {
        list.push(data::CustomPlanningEventResult {
            id: match planning["id"].as_u64() {
                Some(id) => id,
                None => {
                    return HttpResponse::InternalServerError().json(data::Default {
                        msg: String::from("value `id` does not exist"),
                    })
                }
            },

            name: match planning["title"].as_str() {
                Some(name) => String::from(name),
                None => {
                    return HttpResponse::InternalServerError().json(data::Default {
                        msg: String::from("value `title` does not exist"),
                    })
                }
            },
        })
    }

    HttpResponse::Ok().json(list)
}
