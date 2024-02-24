mod banner_gen;

use actix_web::{get, web::Json, App, HttpServer};
use actix_files::NamedFile;
use serde::{Deserialize, Serialize};

const PORT: &str = "5003";

#[derive(Debug, Serialize, Deserialize)]
struct BodyPattern {
    pattern: String,
    color: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RequestBody {
    base_color: String,
    patterns: Vec<BodyPattern>,
}

#[get("/")]
async fn index(request: Json<RequestBody>) -> actix_web::Result<NamedFile> {
    let path = banner_gen::generate_banner(request.into_inner());

    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(format!("127.0.0.1:{}", PORT))?
        .run()
        .await
}
