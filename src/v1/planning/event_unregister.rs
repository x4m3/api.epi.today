use crate::intra::{autologin, check, client};
use crate::v1::data;
use actix_web::{delete, http::StatusCode, web, HttpRequest, HttpResponse, Responder};
use serde_json::Value;

#[delete("/event")]
pub async fn event_unregister(
    req: HttpRequest,
    input: web::Json<data::PlanningEventParams>,
) -> impl Responder {
    let autologin = match autologin::get_from_header(&req) {
        Some(autologin) => autologin,
        _ => {
            return HttpResponse::BadRequest().json(data::Default {
                msg: String::from("no autologin provided"),
            })
        }
    };

    match check::planning_event(
        &input.code_module,
        &input.code_instance,
        &input.code_acti,
        &input.code_event,
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
        "/module/{}/{}/{}/{}/{}/unregister?format=json",
        input.year, input.code_module, input.code_instance, input.code_acti, input.code_event
    );
    let res = match client::post_path_auth(&client, &autologin, &path).await {
        Ok(res) => res,
        Err(_) => {
            return HttpResponse::ServiceUnavailable().json(data::Default {
                msg: String::from("client error"),
            })
        }
    };

    // unregistered
    if res.status() == StatusCode::OK {
        return HttpResponse::Ok().json(data::Default {
            msg: String::from("unregistered"),
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

    match raw_json["error"].as_str() {
        Some(error) => {
            if error == "You cannot unregister from a past activity" {
                // past event
                HttpResponse::BadRequest().json(data::Default {
                    msg: String::from("past event"),
                })
            } else {
                // not registered
                HttpResponse::InternalServerError().json(data::Default {
                    msg: String::from("not registered"),
                })
            }
        }
        None => {
            // generic error
            HttpResponse::InternalServerError().json(data::Default {
                msg: String::from("could not register"),
            })
        }
    }
}
