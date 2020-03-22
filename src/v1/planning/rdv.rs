use crate::intra::{autologin, check, client, format};
use crate::v1::data;
use actix_web::{get, http::StatusCode, web, HttpRequest, HttpResponse, Responder};
use serde_json::Value;

#[get("/rdv")]
pub async fn rdv(req: HttpRequest, input: web::Json<data::PlanningRdvParams>) -> impl Responder {
    let autologin = match autologin::get_from_header(&req) {
        Some(autologin) => autologin,
        _ => {
            return HttpResponse::BadRequest().json(data::Default {
                msg: String::from("no autologin provided"),
            })
        }
    };

    match check::planning_rdv(
        &input.code_module,
        &input.code_instance,
        &input.code_acti,
        &input.email,
    ) {
        None => (),
        Some(error) => {
            return HttpResponse::BadRequest().json(data::Default { msg: error });
        }
    }

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

    let path = format!(
        "/module/{}/{}/{}/{}/rdv/?format=json",
        input.year, input.code_module, input.code_instance, input.code_acti
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
            msg: String::from("could not get rdv information"),
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

    // Extract rdv title
    let rdv_title = match raw_json["events"][0]["title"].as_str() {
        Some(title) => String::from(title),
        None => {
            return HttpResponse::InternalServerError().json(data::Default {
                msg: String::from("value `events.0.title` does not exist"),
            })
        }
    };

    // Find slot where email address matches and extract start and end times
    let mut time_start = String::new();
    let mut time_end = String::new();

    let slots = match raw_json["slots"].as_array() {
        Some(slots) => slots,
        None => {
            return HttpResponse::InternalServerError().json(data::Default {
                msg: String::from("value `slots` is not an array"),
            })
        }
    };
    for slot in slots {
        // println!("slot : {}", slot["slots"]);

        let slots = match slot["slots"].as_array() {
            Some(slots) => slots,
            None => {
                return HttpResponse::InternalServerError().json(data::Default {
                    msg: String::from("value `slots.[].slots` is not an array"),
                })
            }
        };
        for slot in slots {
            // println!("---- slot : {}", slot);

            let master_login = match slot["master"]["login"].as_str() {
                Some(login) => login,
                None => "null",
            };
            println!(
                "--------------------------------- master login: {}",
                master_login
            );
            if master_login == input.email {
                println!("user is group master!");

                time_start = match format::rdv_time_start(&slot["date"]) {
                    Some(time_start) => time_start,
                    None => {
                        return HttpResponse::InternalServerError().json(data::Default {
                            msg: String::from(
                                "value start of `slots.[].slots.[].date` failed to extract",
                            ),
                        });
                    }
                };
                println!("time_start {}", time_start);

                time_end = match format::rdv_time_end(&slot) {
                    Some(time_end) => time_end,
                    None => {
                        return HttpResponse::InternalServerError().json(data::Default {
                            msg: String::from(
                                "value end of `slots.[].slots.[].date` failed to extract",
                            ),
                        });
                    }
                };
                println!("time_end {}", time_end);
            } else {
                println!("user is not group master");
            }
        }
    }

    if time_start.len() == 0 || time_end.len() == 0 {
        return HttpResponse::InternalServerError().json(data::Default {
            msg: String::from("failed to extract start and end of rdv"),
        });
    }

    let rdv = data::PlanningRdvResult {
        title: rdv_title,
        time_start: time_start,
        time_end: time_end,
    };

    HttpResponse::Ok().json(rdv)
}
