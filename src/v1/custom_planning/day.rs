use crate::intra::{autologin, check, client, format};
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

    for event in &raw_json {
        list.push(data::CustomPlanningEventResult {
            calendar_id: match event["id_calendar"].as_u64() {
                Some(calendar_id) => calendar_id,
                None => {
                    return HttpResponse::InternalServerError().json(data::Default {
                        msg: String::from("value `id_calendar` does not exist"),
                    })
                }
            },

            event_id: match event["id"].as_u64() {
                Some(event_id) => event_id,
                None => {
                    return HttpResponse::InternalServerError().json(data::Default {
                        msg: String::from("value `event_id` does not exist"),
                    })
                }
            },

            title: match event["title"].as_str() {
                Some(title) => String::from(title),
                None => {
                    return HttpResponse::InternalServerError().json(data::Default {
                        msg: String::from("value `title` does not exist"),
                    })
                }
            },

            room: match event["location"].as_str() {
                Some(room) => match format::room(room) {
                    Some(room) => room,
                    None => {
                        return HttpResponse::InternalServerError().json(data::Default {
                            msg: String::from("formatting value `location` failed"),
                        })
                    }
                },
                None => String::from("At the bar 🍺"),
            },

            time_start: match event["start"].as_str() {
                Some(start) => match format::time(&start) {
                    Some(start) => start,
                    None => {
                        return HttpResponse::InternalServerError().json(data::Default {
                            msg: String::from("formatting value `start` failed"),
                        })
                    }
                },
                None => {
                    return HttpResponse::InternalServerError().json(data::Default {
                        msg: String::from("value `start` does not exist"),
                    })
                }
            },

            time_end: match event["end"].as_str() {
                Some(end) => match format::time(&end) {
                    Some(end) => end,
                    None => {
                        return HttpResponse::InternalServerError().json(data::Default {
                            msg: String::from("formatting value `end` failed"),
                        })
                    }
                },
                None => {
                    return HttpResponse::InternalServerError().json(data::Default {
                        msg: String::from("value `end` does not exist"),
                    })
                }
            },

            teacher: match event["maker"]["title"].as_str() {
                Some(teacher) => String::from(teacher),
                None => {
                    return HttpResponse::InternalServerError().json(data::Default {
                        msg: String::from("value `maker.title` does not exist"),
                    })
                }
            },

            registration_status: match event["event_registered"].as_str() {
                Some(registration_status) => {
                    if registration_status == "registered" {
                        true
                    } else {
                        false
                    }
                }
                None => false,
            },
        })
    }

    HttpResponse::Ok().json(list)
}
