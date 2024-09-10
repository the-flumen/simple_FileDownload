use actix_web::{get, web, App, HttpServer, HttpResponse, Result};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use zip::write::FileOptions;
use std::collections::HashMap;
use walkdir::WalkDir;

#[get("/file")]
async fn get_file(query: web::Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let file_path_str = match query.get("path") {
        Some(path) => path,
        None => return Ok(HttpResponse::BadRequest().body("Missing 'path' query parameter")),
    };

    let file_path: PathBuf = PathBuf::from(file_path_str);

    // 파일이 존재하는지 확인
    if !file_path.exists() {
        return Ok(HttpResponse::NotFound().body("File not found"));
    }

    // 파일을 열고 읽기
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(_) => return Ok(HttpResponse::InternalServerError().body("Could not open file")),
    };

    let mut file_contents = Vec::new();
    if let Err(_) = file.read_to_end(&mut file_contents) {
        return Ok(HttpResponse::InternalServerError().body("Could not read file"));
    }

    // 파일 반환
    Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .body(file_contents))
}


// 여러 파일을 압축하여 반환하는 함수
// /files?path=sjljdlf/slkdjlfj,lsdl/asldf
// , 를 구분된 파라미터
#[get("/files")]
async fn get_files(query: web::Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let file_paths_str = match query.get("path") {
        Some(paths) => paths,
        None => return Ok(HttpResponse::BadRequest().body("Missing 'paths' query parameter")),
    };

    let file_paths: Vec<&str> = file_paths_str.split(',').collect();

    let mut zip_buffer: Vec<u8> = Vec::new();
    {
        let mut zip_writer = zip::ZipWriter::new(std::io::Cursor::new(&mut zip_buffer));

        for file_path_str in file_paths {
            let file_path = Path::new(file_path_str);
            if !file_path.exists() {
                return Ok(HttpResponse::NotFound().body(format!("File not found: {}", file_path_str)));
            }

            let mut file = match File::open(&file_path) {
                Ok(f) => f,
                Err(_) => return Ok(HttpResponse::InternalServerError().body("Could not open file")),
            };

            let mut file_contents = Vec::new();
            if let Err(_) = file.read_to_end(&mut file_contents) {
                return Ok(HttpResponse::InternalServerError().body("Could not read file"));
            }

            zip_writer
                .start_file(file_path.file_name().unwrap().to_str().unwrap(), FileOptions::default())
                .expect("Failed to add file to zip");
            zip_writer.write_all(&file_contents).expect("Failed to write file contents to zip");
        }

        zip_writer.finish().expect("Failed to finalize zip");
    } 

    Ok(HttpResponse::Ok()
        .content_type("application/zip")
        .body(zip_buffer))
}

// 폴더를 압축하여 반환하는 API
// /folder?path=slasjdlfaj/
// slasjdlfaj/ 하위 파일들을 압축하여 리턴
#[get("/folder")]
async fn get_folder(query: web::Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let folder_path_str = match query.get("path") {
        Some(path) => path,
        None => return Ok(HttpResponse::BadRequest().body("Missing 'path' query parameter")),
    };

    let folder_path = Path::new(folder_path_str);
    if !folder_path.exists() {
        return Ok(HttpResponse::NotFound().body("Folder not found"));
    }

    let mut zip_buffer: Vec<u8> = Vec::new();
    {
        let mut zip_writer = zip::ZipWriter::new(std::io::Cursor::new(&mut zip_buffer));
        let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored);

        // 폴더 하위 파일 재귀로 압축
        for entry in WalkDir::new(folder_path) {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_file() {
                let name_in_zip = path.strip_prefix(folder_path).unwrap().to_str().unwrap();

                zip_writer.start_file(name_in_zip, options).expect("Failed to start file in zip");
                
                let mut file = File::open(path).expect("Failed to open file");
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer).expect("Failed to read file");
                zip_writer.write_all(&buffer).expect("Failed to write file to zip");
            } else if path.is_dir() {
                let name_in_zip = path.strip_prefix(folder_path).unwrap().to_str().unwrap();
                zip_writer.add_directory(name_in_zip, options).expect("Failed to add directory to zip");
            }
        }

        zip_writer.finish().expect("Failed to finalize zip");
    }

    Ok(HttpResponse::Ok()
        .content_type("application/zip")
        .body(zip_buffer))
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_file)
            .service(get_files)
            .service(get_folder)
    })
    .bind("127.0.0.1:9999")?
    .run()
    .await
}
