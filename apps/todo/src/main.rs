use actix_web::{App, HttpServer, get, web};

#[get("/hello")]
async fn hello() -> String {
    "Hello.".to_string()
}

#[get("/hello/{name}")]
async fn hello_name(name: web::Path<String>) -> String {
    // "Hello.".to_string()
    format!("Hello, {name}.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(hello_name))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
    
}
