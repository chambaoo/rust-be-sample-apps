use actix_web::{App, HttpResponse, HttpServer, get, web};
use askama::Template;
use askama_actix::TemplateToResponse;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    name: String,
}

#[derive(Template)]
#[template(path = "todo.html")]
struct TodoTemplate {
    tasks: Vec<String>,
}

#[get("/")]
async fn todo() -> HttpResponse {
    let tasks = vec![
        "task1".to_string(),
        "task2".to_string(),
        "task3".to_string(),
    ];
    let todo = TodoTemplate { tasks };
    todo.to_response()
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
    HttpServer::new(|| App::new().service(hello).service(hello_name).service(todo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
