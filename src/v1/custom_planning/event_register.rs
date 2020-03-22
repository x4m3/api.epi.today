use crate::intra::{autologin, client};
use crate::v1::data;
use actix_web::{http::StatusCode, put, web, HttpRequest, HttpResponse, Responder};

#[put("/event")]
pub async fn event_register(
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
        "/planning/{}/{}/subscribe?format=json",
        input.calendar_id, input.event_id
    );
    let res = match client::get_path_auth(&client, &autologin, &path).await {
        Ok(res) => res,
        Err(_) => {
            return HttpResponse::ServiceUnavailable().json(data::Default {
                msg: String::from("client error"),
            })
        }
    };

    // could not register
    if res.status() != StatusCode::OK {
        return HttpResponse::Forbidden().json(data::Default {
            msg: String::from("could not register"),
        });
    }

    // registered
    HttpResponse::Ok().json(data::Default {
        msg: String::from("registered"),
    })
}
