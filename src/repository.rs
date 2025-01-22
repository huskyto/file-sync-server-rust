
use std::collections::HashMap;

use crate::util::Util;
use crate::model::FileData;
use crate::model::FileChange;
use crate::model::ChangeType;
use crate::model::FileDefinition;
use crate::model::RevisionHistory;
use crate::model::FileRepositoryState;
use crate::io_manager::IOManager;
use crate::io_manager::FolderIOManager;


pub struct FileRepository {
    state: FileRepositoryState,
    io_manager: FolderIOManager,    // TODO: make it generic
    contents: HashMap<String, FileDefinition>
}
impl FileRepository {
    pub fn new() -> Self {
        Self {
            state: FileRepositoryState { current_revision: 0, history: RevisionHistory {
                revisions: vec![] }
            },
            io_manager: FolderIOManager { },
            contents: HashMap::new(),
        }
    }
    pub async fn get_file(&self, id: &str) -> Result<FileData, String> {
        let file_def = self.get(id).expect("File not found");
        match self.io_manager.get_file_content(&file_def).await {
            Ok(content) => {
                Ok(FileData::new(file_def, content))
            },
            Err(e) => Err(e)
        }
    }
    pub async fn create_empty(&mut self, file_def: &FileDefinition) -> Result<bool, String> {
        if self.exists_named(file_def) {
            Err("File already exists".to_string())
        }
        else {
            let mut file_definition = file_def.clone();
            file_definition.id = Util::new_id();
            file_definition.size = 0;
            match self.io_manager.create_empty(&file_definition).await {
                Ok(_) => {
                    let change = FileChange::new(file_definition, ChangeType::Create);
                    self.state.add_revision(change);
                    Ok(true)
                },
                Err(e) => Err(e)
            }
        }
    }
    pub async fn update(&mut self, file_data: &FileData) -> Result<bool, String> {
        let file_def = &file_data.definition;
        if self.contents.contains_key(&file_def.id) {
            FileChange::new(file_def.clone(), ChangeType::Update)
        }
        else {
            FileChange::new(file_def.clone(), ChangeType::Create)
        };
        match self.io_manager.store_file_content(file_data).await {
            Ok(_) => {
                let mut updated_def = file_def.clone();
                updated_def.size = file_data.content.len() as u64;
                updated_def.checksum = Some(Util::checksum(&file_data.content));
                let change = FileChange::new(updated_def, ChangeType::Update);
                self.contents.insert(file_def.id.to_string(), file_def.clone());
                self.state.add_revision(change);
                Ok(true)
            },
            Err(e) => {
                Err(e.to_string())
            }
        }
    }
    pub async fn delete(&mut self, id: &str) -> Option<FileDefinition> {
        let res = self.contents.remove(id);
        if let Some(file) = &res {
            match self.io_manager.delete_file(file).await {
                Ok(_) => {
                    let change = FileChange::new(file.clone(), ChangeType::Delete);
                    self.state.add_revision(change);
                    res
                },
                Err(e) => {
                    println!("Error: {}", e);
                    return None;
                }
            }
        }
        else {
            None
        }
    }
    pub fn get_revision(&self) -> u64 {
        self.state.current_revision
    }
    pub fn exists(&self, id: &str) -> bool {
        self.contents.contains_key(id)
    }
    pub fn exists_named(&self, file_def: &FileDefinition) -> bool {
        self.contents.values().any(|f| f.name == file_def.name && f.path == file_def.path)
    }
    pub fn get(&self, id: &str) -> Option<FileDefinition> {
        self.contents.get(id).cloned()
    }
}
