use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    status_code: u16,
    status_msg: String,
    autologin: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Autologin {
    autologin: String,
}

async fn index_post(item: web::Json<Autologin>) -> impl Responder {
    println!("model: {:?}", &item); // show data received

    let ret: Test = Test {
        status_code: 200,
        status_msg: String::from("okay"),
        autologin: item.autologin.clone(), // allocate memory and copy
    };
    println!("model: {:?}", &ret);

    web::Json(ret)
}

async fn index_get() -> impl Responder {
    let mut list: Vec<Test> = Vec::new(); // list

    list.push(Test {
        status_code: 200,
        status_msg: String::from("okay"),
        autologin: String::from("tek"),
    });
    list.push(Test {
        status_code: 404,
        status_msg: String::from("not found"),
        autologin: String::from("tek"),
    });
    list.push(Test {
        status_code: 500,
        status_msg: String::from("server error"),
        autologin: String::from("tek"),
    });

    println!("model: {:?}", &list);
    web::Json(list)
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
