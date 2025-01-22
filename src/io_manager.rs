
use rocket::tokio;
use rocket::tokio::fs::File;
use rocket::tokio::io::AsyncReadExt;
use rocket::tokio::io::AsyncWriteExt;

use crate::util::Util;
use crate::model::FileData;
use crate::model::FileDefinition;


#[allow(async_fn_in_trait)]
pub trait IOManager {
    async fn get_file_content(&self, file: &FileDefinition) -> Result<Vec<u8>, String>;
    async fn store_file_content(&self, file: &FileData) -> Result<(), String>;
    async fn create_empty(&self, file: &FileDefinition) -> Result<bool, String>;
    async fn delete_file(&self, file_def: &FileDefinition) -> Result<bool, String>;
}

pub struct FolderIOManager;
impl IOManager for FolderIOManager {
    async fn get_file_content(&self, file: &FileDefinition) -> Result<Vec<u8>, String> {
        let full_path = Util::full_path(file);
        match File::open(&full_path).await {
            Ok(mut file) => {
                let mut content = Vec::new();
                file.read_to_end(&mut content).await.expect("Unable to read file");
                Ok(content)
            },
            Err(_) => {
                Err("File not found.".to_string())
            }
        }
    }

    async fn store_file_content(&self, file_data: &FileData) -> Result<(), String> {
        if !Util::validate_path(&file_data.definition.path).await {
            return Err("Invalid path.".to_string());
        }

        let full_path_str = Util::full_path(&file_data.definition);

        let mut file = File::create(&full_path_str).await.expect("Unable to create file");
        match file.write_all(&file_data.content).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string())
        }
    }
    
    async fn create_empty(&self, file_def: &FileDefinition) -> Result<bool, String> {
        if !Util::validate_path(&file_def.path).await {
            return Err("Invalid path.".to_string());
        }

        let full_path_str = Util::full_path(file_def);
        match File::create(&full_path_str).await {
            Ok(_) => Ok(true),
            Err(e) => Err(e.to_string())
        }
    }
    
    async fn delete_file(&self, file_def: &FileDefinition) -> Result<bool, String> {
        if !Util::validate_path(&file_def.path).await {
            return Err("Invalid path.".to_string());
        }

        let full_path_str = Util::full_path(file_def);

        match tokio::fs::remove_file(&full_path_str).await {
            Ok(_) => Ok(true),
            Err(e) => Err(e.to_string())
        }
    }
}