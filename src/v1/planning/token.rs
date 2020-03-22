use crate::intra::{autologin, client, format::check};
use crate::v1::data;
use actix_web::{put, web, HttpRequest, HttpResponse, Responder};
use serde_json::Value;

#[put("/token")]
pub async fn token(
    req: HttpRequest,
    input: web::Json<data::PlanningTokenParams>,
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

    // TODO: before submitting request, check "present" field in GET /registered?format=json

    // Construct data to be sent
    let data = data::PlanningSubmitTokenParams {
        token: input.token.to_string(),
    };

    let path = format!(
        "/module/{}/{}/{}/{}/{}",
        input.year, input.code_module, input.code_instance, input.code_acti, input.code_event
    );
    let res = match client::post_token(&client, &autologin, &path, &data).await {
        Ok(res) => res,
        Err(_) => {
            return HttpResponse::ServiceUnavailable().json(data::Default {
                msg: String::from("client error"),
            })
        }
    };

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
            // return error from intra
            return HttpResponse::InternalServerError().json(data::Default {
                msg: String::from(error),
            });
        }
        None => {
            // if object "error" is not there, token has been registered
            return HttpResponse::Ok().json(data::Default {
                msg: String::from("token registered"),
            });
        }
    };
}
