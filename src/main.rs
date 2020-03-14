use actix_web::{web, App, HttpResponse, HttpServer, Responder}; // for web server
use reqwest; // for http client
use serde::{Deserialize, Serialize}; // for json serial / de-serial
use std::time::Duration; // for timeout

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

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

async fn simple_json() -> Result<(), reqwest::Error> {
    println!("test");

    let request_url = format!("https://intra.epitech.eu/auth-5095dbdcd778bdf9bfee368f2729c84bd357c1ea/planning/4686/events?format=json&start=2020-03-11&end=2020-03-11");

    let timeout = Duration::new(5, 0);
    let client = reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .timeout(timeout)
        .build()?;
    let res = client.get(&request_url).send().await?;

    println!("http return code: {}", res.status());

    let body = res.text().await?;
    println!("Body:\n\n{}", body);

    Ok(())
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

    simple_json().await;

    web::Json(list)
}

/// Displays API documentation
///
/// Documentation is rendered with redoc
///
/// Documentaion file is hosted on GitHub
async fn root_doc() -> HttpResponse {
    HttpResponse::Ok()
        // set as utf8 html file
        .content_type("text/html; charset=utf-8")
        // and no need to have the html file at runtime
        .body(include_str!("../doc/doc.html"))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").route(web::get().to(root_doc)))
            .service(
                web::scope("/post")
                    .route("/test", web::post().to(index_post))
                    .route("/test2", web::get().to(root_doc)),
            )
            .service(web::scope("/get").route("/test", web::get().to(index_get)))
    })
    .bind("127.0.0.1:8003")
    .expect("could not start server on ip/port")
    .run()
    .await
}
