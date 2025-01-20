
use std::path::Path;

use rocket::tokio::fs;

use crate::model::FileData;
use crate::config::Config;


pub struct Util;
impl Util {
    pub async fn validate_path(path: &str) -> bool {
        let path = Path::new(&Config::get_base_path()).join(path);
        if !path.exists() {
            fs::create_dir_all(&path).await.expect("Unable to create directory");
        }

        path.exists()
    }
    pub fn validate_file(full_path: &str) -> bool {
        let path = Path::new(full_path);
        path.exists()
    }
    pub fn full_path(file_data: &FileData) -> String {
        let path = Path::new(&Config::get_base_path())
                    .join(&file_data.path)
                    .join(&file_data.name);

        path.to_str().expect("Invalid path").to_string()
    }
    pub fn full_path_with(path: &str, name: &str) -> String {
        let path = Path::new(&Config::get_base_path())
                    .join(path)
                    .join(name);

        path.to_str().expect("Invalid path").to_string()
    }
    pub fn split_full_path(full_path: &str) -> (String, String) {
        let _path = Path::new(full_path);
        let path = _path.parent().expect("Invalid path")
                    .to_str().expect("Invalid path")
                    .to_string();
        let name = _path.file_name().expect("Invalid file name")
                .to_str().expect("Invalid file name")
                .to_string();

        (path, name)
    }
}
