use actix_web::{App, HttpResponse, HttpServer, get, web};
use askama::Template;
use askama_actix::TemplateToResponse;
use sqlx::{Row, SqlitePool};

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
async fn todo(pool: web::Data<SqlitePool>) -> HttpResponse {
    let rows = sqlx::query("SELECT task FROM tasks;")
        .fetch_all(pool.as_ref())
        .await
        .unwrap();

    let tasks: Vec<String> = rows
        .iter()
        .map(|row| row.get::<String, _>("task"))
        .collect();

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
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    sqlx::query("CREATE TABLE TASKS (task TEXT)")
        .execute(&pool)
        .await
        .unwrap();

    sqlx::query("INSERT INTO tasks (task) VALUES ('TASK1')")
        .execute(&pool)
        .await
        .unwrap();

    sqlx::query("INSERT INTO tasks (task) VALUES ('TASK2')")
        .execute(&pool)
        .await
        .unwrap();

    sqlx::query("INSERT INTO tasks (task) VALUES ('TASK3')")
        .execute(&pool)
        .await
        .unwrap();
    // HttpServer::new(move || App::new().service(hello).service(hello_name).service(todo))
    HttpServer::new(move || {
        App::new()
            .service(hello)
            .service(hello_name)
            .service(todo)
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
