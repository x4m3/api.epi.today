use crate::date;
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

    let full_date = match date::check_yyyy_mm_dd(&input.date) {
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
        "/planning/{}/events?format=json&start={}&end={}",
        input.calendar_id, formatted_date, formatted_date
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

    for planning in &raw_json {
        list.push(data::CustomPlanningEventResult {
            calendar_id: match planning["id_calendar"].as_u64() {
                Some(calendar_id) => calendar_id,
                None => {
                    return HttpResponse::InternalServerError().json(data::Default {
                        msg: String::from("value `id_calendar` does not exist"),
                    })
                }
            },

            event_id: match planning["id"].as_u64() {
                Some(event_id) => event_id,
                None => {
                    return HttpResponse::InternalServerError().json(data::Default {
                        msg: String::from("value `event_id` does not exist"),
                    })
                }
            },

            title: match planning["title"].as_str() {
                Some(title) => String::from(title),
                None => {
                    return HttpResponse::InternalServerError().json(data::Default {
                        msg: String::from("value `title` does not exist"),
                    })
                }
            },

            // TODO: correct formatting of room
            room: match planning["location"].as_str() {
                Some(room) => String::from(room),
                None => String::from("At the bar"),
            },

            // TODO: correct formatting of time
            time_start: match planning["start"].as_str() {
                Some(start) => String::from(start),
                None => {
                    return HttpResponse::InternalServerError().json(data::Default {
                        msg: String::from("value `start` does not exist"),
                    })
                }
            },

            // TODO: correct formatting of time
            time_end: match planning["end"].as_str() {
                Some(time_end) => String::from(time_end),
                None => {
                    return HttpResponse::InternalServerError().json(data::Default {
                        msg: String::from("value `end` does not exist"),
                    })
                }
            },

            teacher: match planning["maker"]["title"].as_str() {
                Some(teacher) => String::from(teacher),
                None => {
                    return HttpResponse::InternalServerError().json(data::Default {
                        msg: String::from("value `maker.title` does not exist"),
                    })
                }
            },

            registration_status: true,
        })
    }

    HttpResponse::Ok().json(list)
}
