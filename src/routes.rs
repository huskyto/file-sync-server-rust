

use std::path::PathBuf;
use rocket::http::Status;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket::tokio::fs::File;
use rocket::tokio::io::AsyncWriteExt;
use rocket::tokio::io::AsyncReadExt;
use base64::prelude::*;

use crate::model::FileData;
use crate::util::Util;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[post("/file", data = "<fd>")]
pub async fn post_file(fd: Json<FileData>) -> Status {
    let file_data = fd.into_inner();
    if !Util::validate_path(&file_data.path).await {
        return Status::Conflict;
    }

    let full_path_str = Util::full_path(&file_data);
    let file_content = BASE64_STANDARD.decode(file_data.content).expect("Invalid encoded file");
    let mut file = File::create(&full_path_str).await.expect("Unable to create file");
    file.write_all(&file_content).await.expect("Unable to write data to file");

    Status::Created
}

#[get("/file/<path_buf..>")]
pub async fn get_file(path_buf: PathBuf) -> Result<Json<FileData>, NotFound<String>> {
    let full_path = path_buf.to_str().expect("Invalid path").to_string();
    match File::open(&full_path).await {
        Ok(mut file) => {
            let mut content = Vec::new();
            file.read_to_end(&mut content).await.expect("Unable to read file");
            let content = BASE64_STANDARD.encode(&content);
            let (path, filename) = Util::split_full_path(&full_path);
            let file_data = FileData::new(filename, path, content);

            Ok(Json(file_data))
        },
        Err(_) => {
            Err(NotFound("File not found.".to_string()))
        }
    }
}
