use actix_web::{App, HttpResponse, HttpServer, get, web};
use askama::Template;
use askama_actix::TemplateToResponse;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}
#[get("/hello")]
async fn hello() -> String {
    "Hello.".to_string()
}

#[get("/hello/{name}")]
async fn hello_name(name: web::Path<String>) -> HttpResponse {
    let hello_name = HelloTemplate {
        name: name.into_inner(),
    };
    hello_name.to_response()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(hello_name))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
