use actix_web::{App, HttpResponse, HttpServer, get, post, web};
use askama::Template;
use askama_actix::TemplateToResponse;
use sqlx::{Row, SqlitePool};
use std::env;

// #[derive(Template)]
// #[template(path = "hello.html")]
// struct HelloTemplate {
//     name: String,
// }

#[derive(Template)]
#[template(path = "todo.html")]
struct TodoTemplate {
    tasks: Vec<String>,
}

#[derive(serde::Deserialize)]
struct Task {
    id: Option<String>,
    task: Option<String>,
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

#[post("/update")]
async fn update(pool: web::Data<SqlitePool>, form: web::Form<Task>) -> HttpResponse {
    let task = form.into_inner();

    match task.id {
        Some(id) => {
            sqlx::query("DELETE FROM tasks WHERE task = ?")
                .bind(id)
                .execute(pool.as_ref())
                .await
                .unwrap();
        }
        None => {}
    }
    match task.task {
        Some(task) if task != "" => {
            sqlx::query("INSERT INTO tasks (task) VALUES (?)")
                .bind(task)
                .execute(pool.as_ref())
                .await
                .unwrap();
        }
        _ => {}
    }

    HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish()
}

// #[get("/hello")]
// async fn hello() -> String {
//     "Hello.".to_string()
// }

// #[get("/hello/{name}")]
// async fn hello_name(name: web::Path<String>) -> HttpResponse {
//     let hello_name = HelloTemplate {
//         name: name.into_inner(),
//     };
//     hello_name.to_response()
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let host_key = "HOST";
    let port_key = "PORT";
    let default_port = 8080;
    let default_host = "127.0.0.1";

    let host = env::var(host_key).unwrap_or_else(|_| default_host.to_string());
    let port: u16 = env::var(port_key)
        .unwrap_or_else(|_| default_port.to_string())
        .parse()
        .unwrap();

    dbg!(&host);
    dbg!(port);

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
            // .service(hello)
            // .service(hello_name)
            .service(todo)
            .service(update)
            .app_data(web::Data::new(pool.clone()))
    })
    .bind((host, port))?
    .run()
    .await
}
