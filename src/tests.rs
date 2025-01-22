
#[cfg(test)]
mod util_tests {
    use rocket::tokio::fs;
    use crate::model::FileDefinition;
    use crate::util::Util;

    #[rocket::async_test]
    async fn test_validate_path() {
        let path = "test_dir";
        let result = Util::validate_path(path).await;
        assert!(result);
    }

    #[rocket::async_test]
    async fn test_validate_file() {
        let path = "test_file.txt";
        fs::File::create(path).await.expect("Unable to create test file");
        let result = Util::validate_file(path);
        assert!(result);
        fs::remove_file(path).await.expect("Unable to remove test file");
    }

    #[test]
    fn test_full_path() {
        let file_def = FileDefinition {
            id: Some("test_id".to_string()),
            name: "test_file.txt".to_string(),
            path: "test_dir".to_string(),
            checksum: None,
            size: Some(0),
        };
        let full_path = Util::full_path(&file_def);
        assert!(full_path.contains("test_id"));
    }

    #[test]
    fn test_split_full_path() {
        let full_path = "test_dir/test_file.txt";
        let (path, name) = Util::split_full_path(full_path);
        assert_eq!(path, "test_dir");
        assert_eq!(name, "test_file.txt");
    }

    #[test]
    fn test_checksum() {
        let content = b"test content".to_vec();
        let checksum = Util::checksum(&content);
        assert_eq!(checksum, "9473fdd0d880a43c21b7778d34872157");
    }

    #[test]
    fn test_new_id() {
        let id = Util::new_id();
        assert_eq!(id.len(), 16);
    }
}

#[cfg(test)]
mod io_manager_tests {
    use crate::model::FileData;
    use crate::model::FileDefinition;
    use crate::io_manager::IOManager;
    use crate::io_manager::FolderIOManager;

    #[rocket::async_test]
    async fn test_create_empty_file() {
        let file_def = FileDefinition {
            id: Some("test_id".to_string()),
            name: "test_file.txt".to_string(),
            path: "test_dir".to_string(),
            checksum: None,
            size: Some(0),
        };
        let io_manager = FolderIOManager;
        let result = io_manager.create_empty(&file_def).await;
        assert!(result.is_ok());
    }

    #[rocket::async_test]
    async fn test_store_file_content() {
        let file_def = FileDefinition {
            id: Some("test_id".to_string()),
            name: "test_file.txt".to_string(),
            path: "test_dir".to_string(),
            checksum: None,
            size: Some(0),
        };
        let file_data = FileData {
            definition: file_def.clone(),
            content: b"test content".to_vec(),
        };
        let io_manager = FolderIOManager;
        let result = io_manager.store_file_content(&file_data).await;
        assert!(result.is_ok());
    }

    #[rocket::async_test]
    async fn test_delete_file() {
        let file_def = FileDefinition {
            id: Some("test_id".to_string()),
            name: "test_file.txt".to_string(),
            path: "test_dir".to_string(),
            checksum: None,
            size: Some(0),
        };
        let io_manager = FolderIOManager;
        io_manager.create_empty(&file_def).await.expect("Unable to create test file");
        let result = io_manager.delete_file(&file_def).await;
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod repository_tests {
    use crate::util::Util;
    use crate::model::FileData;
    use crate::model::FileDefinition;
    use crate::repository::FileRepository;

    #[rocket::async_test]
    async fn test_create_empty_file_in_repository() {
        let mut repository = FileRepository::new();
        let file_def = FileDefinition {
            id: Some("test_id".to_string()),
            name: "test_file.txt".to_string(),
            path: "test_dir".to_string(),
            checksum: None,
            size: Some(0),
        };
        let result = repository.create_empty(&file_def).await;
        assert!(result.is_ok());
        assert!(repository.exists(&result.unwrap()));
    }

    #[rocket::async_test]
    async fn test_update_file_in_repository() {
        let mut repository = FileRepository::new();
        let file_def = FileDefinition {
            id: Some("test_id".to_string()),
            name: "test_file.txt".to_string(),
            path: "test_dir".to_string(),
            checksum: None,
            size: Some(0),
        };
        repository.create_empty(&file_def).await.expect("Unable to create empty file");
        let file_data = FileData {
            definition: file_def.clone(),
            content: b"updated content".to_vec(),
        };
        let result = repository.update(&file_data).await;
        assert!(result.is_ok());
        let updated_file = repository.get_definition(&file_def.id.unwrap()).expect("File not found");
        assert_eq!(updated_file.size.unwrap(), file_data.content.len() as u64);
        assert_eq!(updated_file.checksum, Some(Util::checksum(&file_data.content)));
    }

    #[rocket::async_test]
    async fn test_delete_file_in_repository() {
        let mut repository = FileRepository::new();
        let file_def = FileDefinition {
            id: Some("test_id".to_string()),
            name: "test_file.txt".to_string(),
            path: "test_dir".to_string(),
            checksum: None,
            size: Some(0),
        };
        let created_id = repository.create_empty(&file_def).await.expect("Unable to create empty file");
        let result = repository.delete(&created_id).await;
        assert!(result.is_some());
        assert!(!repository.exists(&created_id));
    }
}


