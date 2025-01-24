
use std::path::Path;

use rand::Rng;
use rocket::tokio::fs;
use rand::distributions::Alphanumeric;

use crate::config::Config;
use crate::model::FileDefinition;


pub struct Util;
impl Util {
    pub async fn validate_path(path: &str) -> bool {
        let path = Path::new(&Config::get_base_path()).join(path);
        if !path.exists() {
            fs::create_dir_all(&path).await.expect("Unable to create directory");
        }

        path.exists()
    }
    pub fn full_path(file_def: &FileDefinition) -> String {
        let path = Path::new(&Config::get_base_path())
                    .join(file_def.id.as_ref().expect("No id in File Definition"));

        path.to_str().expect("Invalid path").to_string()
    }

    pub fn checksum(content: &Vec<u8>) -> String {
        let digest = md5::compute(content);
        format!("{:x}", digest)
    }
    pub fn new_id() -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect()
    }
}
