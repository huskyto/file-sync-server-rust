
use std::path::Path;
use std::collections::HashMap;

use crate::util::Util;
use crate::config::Config;
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
    pub fn load_default() -> FileRepository {
        let (state, contents) = match Self::load_state() {
            Ok(state) => state,
            Err(_) => {
                println!("No repository state to load. Creating new empty one...");
                (
                    FileRepositoryState { current_revision: 0, history: RevisionHistory {
                        revisions: Vec::new() }
                    },
                    HashMap::new()
                )
            },
        };

        Self {
            state,
            io_manager: FolderIOManager { },
            contents,
        }
    }
    fn add_change(&mut self, change: FileChange) {
        self.state.add_revision(change);
        let _ = self.save_state();
    }

    pub fn get_definition(&self, id: &str) -> Option<FileDefinition> {
        self.contents.get(id).cloned()
    }

    pub async fn get_file_data(&self, id: &str) -> Result<FileData, String> {
        let file_def = match self.get_definition(id) {
            Some(res) => res,
            None => return Err("File not found".to_string()),
        };
        match self.io_manager.get_file_content(&file_def).await {
            Ok(content) => {
                Ok(FileData::new(file_def, content))
            },
            Err(e) => Err(e)
        }
    }

    pub async fn create_empty(&mut self, file_def: &FileDefinition) -> Result<String, String> {
        if self.exists_named(file_def) {
            Err("File already exists".to_string())
        }
        else {
            let mut file_definition = file_def.clone();
            let new_id = Util::new_id();
            file_definition.id = Some(new_id.clone());
            file_definition.size = Some(0);
            match self.io_manager.create_empty(&file_definition).await {
                Ok(_) => {
                    self.contents.insert(new_id.clone(), file_definition.clone());
                    let change = FileChange::new(file_definition, ChangeType::Create);
                    self.add_change(change);
                    Ok(new_id)
                },
                Err(e) => Err(e)
            }
        }
    }

    pub async fn update(&mut self, file_data: &FileData) -> Result<bool, String> {
        let file_def = &file_data.definition;
        if file_def.id.is_none() {
            return Err("No id in file definition".to_string())
        }

        match self.io_manager.store_file_content(file_data).await {
            Ok(_) => {
                let mut updated_def = file_def.clone();
                updated_def.size = Some(file_data.content.len() as u64);
                updated_def.checksum = Some(Util::checksum(&file_data.content));
                let change = FileChange::new(updated_def.clone(), ChangeType::Update);
                self.contents.insert(file_def.id.clone().expect("No id"), updated_def.clone());
                self.add_change(change);
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
                    self.add_change(change);
                    res
                },
                Err(e) => {
                    println!("Error: {}", e);
                    None
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

    pub fn get_all_entries(&self) -> Vec<&FileDefinition> {
        self.contents.values().into_iter().collect()
    }


    fn save_state(&self) -> Result<(), std::io::Error> {
        let state_str = serde_json::to_string(&self.state)
                    .expect("Repo State serialization error.");
        std::fs::write(Self::get_save_state_path(), state_str.as_bytes())?;

        let content_vec: Vec<&FileDefinition> = self.contents.values().collect();
        let contents_str = serde_json::to_string(&content_vec)
                    .expect("Repo Contents serialization error.");
        std::fs::write(Self::get_save_contents_path(), contents_str.as_bytes())
    }
    fn load_state() -> Result<(FileRepositoryState, HashMap<String, FileDefinition>), std::io::Error> {
        let state_path = Self::get_save_state_path();
        let stored_state: FileRepositoryState =  match std::fs::read(state_path) {
            Ok(data) => {
                serde_json::from_slice(&data).unwrap()
            },
            Err(e) => return Err(e)
        };

        let content_path = Self::get_save_contents_path();
        let stored_content_vec: Vec<FileDefinition> =  match std::fs::read(content_path) {
            Ok(data) => {
                serde_json::from_slice(&data).unwrap()
            },
            Err(e) => return Err(e)
        };
        let stored_content = stored_content_vec.iter()
                    .map(|fd| (fd.id.as_ref().unwrap().clone(), fd.clone()))
                    .collect();

        Ok((stored_state, stored_content))
    }

    fn get_save_state_path() -> String {
        let base_path = &Config::get_base_path();
        let binding = Path::new(base_path).join(".sync-state");
        binding.to_str().unwrap().to_string()
    }
    fn get_save_contents_path() -> String {
        let base_path = &Config::get_base_path();
        let binding = Path::new(base_path).join(".sync-contents");
        binding.to_str().unwrap().to_string()
    }
}
