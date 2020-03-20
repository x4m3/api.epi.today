use crate::intra::client;
use crate::v1::data;
use actix_web::{get, http::StatusCode, HttpResponse, Responder};

#[get("/intra")]
async fn intra() -> impl Responder {
    let client = match client::create_client() {
        Ok(client) => client,
        Err(_) => {
            return HttpResponse::InternalServerError().json(data::Default {
                msg: String::from("could not create intra client"),
            })
        }
    };

    let path = format!("/?format=json");
    let res = match client::get_path(&client, &path).await {
        Ok(res) => res,
        // if request fails, it may be an error from our end or something else
        Err(_) => {
            return HttpResponse::ServiceUnavailable().json(data::Default {
                msg: String::from("error"),
            })
        }
    };

    match res.status() {
        StatusCode::FORBIDDEN => {
            // if intra return 403, that means that intra works
            // (403 because we don't have permission to get data)
            return HttpResponse::Ok().json(data::Default {
                msg: String::from("okay"),
            });
        }
        _ => {
            // otherwise, the intra is (probably) down
            return HttpResponse::InternalServerError().json(data::Default {
                msg: String::from("down"),
            });
        }
    }
}
