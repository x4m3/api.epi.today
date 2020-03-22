use crate::intra::{autologin, client};
use crate::v1::data;
use actix_web::{delete, http::StatusCode, web, HttpRequest, HttpResponse, Responder};

#[delete("/event")]
pub async fn event_unregister(
    req: HttpRequest,
    input: web::Json<data::CustomPlanningEventParams>,
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

    let client = match client::create_client() {
        Ok(client) => client,
        Err(_) => {
            return HttpResponse::InternalServerError().json(data::Default {
                msg: String::from("could not create intra client"),
            })
        }
    };

    let path = format!(
        "/planning/{}/{}/unsubscribe?format=json",
        input.calendar_id, input.event_id
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

    // already unregistered
    if res.status() == StatusCode::INTERNAL_SERVER_ERROR {
        return HttpResponse::InternalServerError().json(data::Default {
            msg: String::from("already unregistered"),
        });
    }

    // past event
    if res.status() == StatusCode::BAD_REQUEST {
        return HttpResponse::BadRequest().json(data::Default {
            msg: String::from("past event"),
        });
    }

    // event does not exist
    if res.status() == StatusCode::FORBIDDEN {
        return HttpResponse::Forbidden().json(data::Default {
            msg: String::from("event does not exist"),
        });
    }

    // generic error
    HttpResponse::InternalServerError().json(data::Default {
        msg: String::from("could not unregister"),
    })
}
