
use serde::Deserialize;
use serde::Serialize;


#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct FileDefinition {
    pub id: String,
    pub name: String,
    pub path: String,
    pub size: u64,
    pub checksum: Option<String>
}
impl FileDefinition {
    pub fn new(id: String, name: String, path: String) -> Self {
        Self {
            id,
            name,
            path,
            size: 0,
            checksum: None,
        }
    }
    pub fn with_checksum(id: String, name: String, path: String, checksum: String) -> Self {
        Self {
            id,
            name,
            path,
            size: 0,
            checksum: Some(checksum),
        }
    }
    pub fn validate(&self) -> bool {
        !self.id.is_empty() && !self.name.is_empty() && !self.path.is_empty()
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct FileChange {
    pub file: FileDefinition,
    pub change: ChangeType
}
impl FileChange {
    pub fn new(file: FileDefinition, change: ChangeType) -> Self {
        Self {
            file,
            change
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ChangePatch {
    pub revision: u64,
    pub changes: Vec<FileChange>
}

#[derive(Serialize, Deserialize)]
pub struct RevisionHistory {
    pub revisions: Vec<FileChange>
}

pub struct FileRepositoryState {
    pub current_revision: u64,
    pub history: RevisionHistory
}
impl FileRepositoryState {
    pub fn add_revision(&mut self, change: FileChange) {
        self.current_revision += 1;
        self.history.revisions.push(change);
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum ChangeType {
    Create,
    Update,
    Delete
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct FileData {
    pub definition: FileDefinition,
    pub content: Vec<u8>,
}
impl FileData {
    pub fn new(definition: FileDefinition, content: Vec<u8>) -> Self {
        Self {
            definition,
            content,
        }
    }
}