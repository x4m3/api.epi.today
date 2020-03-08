use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    status_code: u16,
    status_msg: String,
    autologin: String,
}

async fn index_post(item: web::Json<Test>) -> impl Responder {
    println!("model: {:?}", &item);
    HttpResponse::Ok().json(item.0)
}

async fn index_get() -> impl Responder {
    let item: Test = Test {
        status_code: 200,
        status_msg: String::from("okay"),
        autologin: String::from("https://intra.epitech.eu/..."),
    };
    println!("model: {:?}", &item);
    HttpResponse::Ok().json(web::Json(&item).0)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/post").route("/test", web::post().to(index_post)))
            .service(web::scope("/get").route("/test", web::get().to(index_get)))
    })
    .bind("127.0.0.1:8003")?
    .run()
    .await
}
