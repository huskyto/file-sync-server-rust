
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
    pub fn validate_file(full_path: &str) -> bool {
        let path = Path::new(full_path);
        path.exists()
    }
    pub fn full_path(file_def: &FileDefinition) -> String {
        let path = Path::new(&Config::get_base_path())
                    .join(file_def.id.as_ref().expect("No id in File Definition"));
                    // .join(&file_def.name);
        // let path = Path::new(&Config::get_base_path())
        //             .join(&file_def.path)
        //             .join(&file_def.name);

        path.to_str().expect("Invalid path").to_string()
    }
    // pub fn full_path_with(path: &str, name: &str) -> String {
    //     let path = Path::new(&Config::get_base_path())
    //                 .join(path)
    //                 .join(name);

    //     path.to_str().expect("Invalid path").to_string()
    // }
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
