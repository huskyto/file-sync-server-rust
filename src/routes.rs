

use std::path::PathBuf;
use rocket::http::Status;
use rocket::response::status::Accepted;
use rocket::response::status::BadRequest;
use rocket::response::status::Created;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket::tokio::fs::File;
use rocket::tokio::io::AsyncWriteExt;
use rocket::tokio::io::AsyncReadExt;
use base64::prelude::*;
use rocket::tokio::sync::Mutex;
use std::sync::LazyLock;

use crate::model::FileData;
use crate::model::FileDefinition;
use crate::repository::FileRepository;
use crate::util::Util;


static REPOSITORY: LazyLock<Mutex<FileRepository>> = LazyLock::new(|| Mutex::new(FileRepository::new()));  // TODO load instead


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


#[post("/file", data = "<fd>")]
pub async fn create_empty(fd: Json<FileDefinition>) -> Result<Created<String>, BadRequest<String>> {
    match REPOSITORY.lock().await.create_empty(&fd).await {
        Ok(res) => Ok(Created::new(res)),
        Err(e) => {
            println!("[Error [create_empty]: {e}");
            Err(BadRequest(e))
        }
    }
}

#[put("/file/<file_id>", data = "<content>")]
pub async fn update_file(file_id: String, content: Vec<u8>) -> Result<Accepted<String>, BadRequest<String>> {
    let mut rep_lock = REPOSITORY.lock().await;
    match rep_lock.get(&file_id) {
        Some(file_def) => {
            let file_data = FileData::new(file_def, content);
            match rep_lock.update(&file_data).await {
                Ok(res) => Ok(Accepted(res.to_string())),
                Err(e) => {
                    println!("[Error [create_empty]: {e}");
                    Err(BadRequest(e))
                },
            }
        },
        None => Err(BadRequest("File id doesn't exist".to_string())),
    }
}

