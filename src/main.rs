use actix_web::{get, App, HttpServer};

const PORT: &str = "5003";

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(index)
    }).bind(format!("127.0.0.1:{}", PORT))?
    .run()
    .await
}
