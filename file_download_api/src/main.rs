use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[get("/download")]
async fn download(file_path: web::Query<PathBuf>) -> impl Responder {
    let path = file_path.into_inner();

    if !path.exists() {
        return HttpResponse::NotFound().body("File not found");
    }

    match File::open(&path).await {
        Ok(mut file) => {
            let mut contents = vec![];
            if let Err(_) = file.read_to_end(&mut contents).await {
                return HttpResponse::InternalServerError().body("Error reading file");
            }
            HttpResponse::Ok()
                .content_type("application/octet-stream")
                .body(contents)
        }
        Err(_) => HttpResponse::InternalServerError().body("Error opening file"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(download)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
