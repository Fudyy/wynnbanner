use actix_web::{get, web, App, HttpServer, Responder};
use serde::{Serialize, Deserialize};

const PORT: &str = "5003";

#[derive(Debug, Serialize, Deserialize)]
struct Pattern {
    pattern: String,
    color: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RequestBody {
    base_color: String,
    patterns: Vec<Pattern>,
}

#[get("/")]
async fn index(request: web::Json<RequestBody>) -> impl Responder {
    let mut result = String::from("Base color: ");
    result.push_str(&request.base_color);
    result.push_str("\nPatterns: ");
    for pattern in &request.patterns {
        result.push_str(&format!("{} - {} ", pattern.pattern, pattern.color));
    }
    result
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
