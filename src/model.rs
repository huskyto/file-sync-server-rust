
use serde::Deserialize;
use serde::Serialize;


#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct FileData {
    pub name: String,
    pub path: String,
    pub content: String,
    pub checksum: Option<String>
}
impl FileData {
    pub fn new(name: String, path: String, content: String) -> Self {
        FileData {
            name,
            path,
            content,
            checksum: None
        }
    }
    pub fn with_checksum(name: String, path: String, content: String, checksum: String) -> Self {
        FileData {
            name,
            path,
            content,
            checksum: Some(checksum)
        }
    }
}

