mod banner_gen;

use std::time::Duration;
use actix_rt::time;
use actix_web::{get, web::Json, App, HttpServer};
use actix_files::NamedFile;
use serde::{Deserialize, Serialize};
use actix::spawn;
use std::fs;
use std::io;

const PORT: u16 = 3000;

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
    println!("Generated banner at path: {}", path);
    Ok(NamedFile::open(path)?)
}

fn remove_files_from_path(path: &str) -> io::Result<()> {
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let file_type = entry.file_type()?;
        
        if file_type.is_file() {
            fs::remove_file(entry.path())?;
        }
    }

    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server on port: {}", PORT);
    
    match fs::create_dir("images") {
        Ok(_) => println!("Created images directory"),
        Err(_) => (),
    };

    spawn(async move {
        let mut interval = time::interval(Duration::from_secs(3600));
        loop {
            interval.tick().await;
            println!("Clearing saved images");
            match remove_files_from_path("images/") {
                Ok(_) => println!("Cleared saved images"),
                Err(err) => {
                    println!("Error clearing saved images: {}", err);
                },
            };
        }
    });

    HttpServer::new(|| App::new().service(index))
        .bind(format!("0.0.0.0:{}", PORT))?
        .run()
        .await
}
