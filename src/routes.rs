

use rocket::response::status::Accepted;
use rocket::response::status::BadRequest;
use rocket::response::status::Created;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket::tokio::sync::Mutex;
use std::sync::LazyLock;

use crate::model::FileData;
use crate::model::FileDefinition;
use crate::repository::FileRepository;


static REPOSITORY: LazyLock<Mutex<FileRepository>> = LazyLock::new(|| Mutex::new(FileRepository::new()));  // TODO load instead


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
pub async fn update_file(file_id: &str, content: Vec<u8>) -> Result<Accepted<String>, BadRequest<String>> {
    let mut rep_lock = REPOSITORY.lock().await;
    match rep_lock.get_definition(&file_id) {
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

#[get("/file/<file_id>")]
pub async fn get_file(file_id:  &str) -> Result<Vec<u8>, NotFound<String>> {
    match REPOSITORY.lock().await.get_file_data(file_id).await {
        Ok(res) => {
            Ok(res.content)
        },
        Err(e) => Err(NotFound(e)),
    }
}

#[delete("/file/<file_id>")]
pub async fn delete_file(file_id: &str) -> Result<Accepted<String>, NotFound<String>> {
    match REPOSITORY.lock().await.delete(file_id).await {
        Some(_res) => Ok(Accepted("Deleted".to_string())),
        None => Err(NotFound("File not found".to_string())),
    }
}
