
use crate::model::ChangePatch;
use crate::model::ChangeType;
use crate::model::FileChange;
use crate::model::FileData;
use crate::model::FileDefinition;
use crate::repository::FileRepository;


pub struct Patcher;
impl Patcher {
    pub fn get_patch(rev: u64, file_list: &Vec<FileDefinition>, repository: &FileRepository) -> Option<ChangePatch> {
        if rev == 0 {
            return Self::build_initial_patch(repository)
        }

        None
    }

    fn build_initial_patch(repository: &FileRepository) -> Option<ChangePatch> {
        let revision = repository.get_revision();
        let entries = repository.get_all_entries();
        let changes = entries.iter()
                .map(|d| FileChange::new(d.clone().clone(), ChangeType::Create))
                .collect();

        Some(ChangePatch::new(revision, changes))
    }
}
