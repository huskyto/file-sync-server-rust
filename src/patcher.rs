
use std::collections::HashMap;

use crate::model::FileChange;
use crate::model::ChangeType;
use crate::model::ChangePatch;
use crate::model::FileDefinition;
use crate::repository::FileRepository;


pub struct Patcher;
impl Patcher {
    pub fn get_patch(rev: u64, file_list: &Vec<FileDefinition>, repository: &FileRepository) -> Option<ChangePatch> {
        if rev == 0 {
            Self::build_initial_patch(repository)
        }
        else {
            Self::build_patch_for(rev, file_list, repository)
        }
    }

    fn build_initial_patch(repository: &FileRepository) -> Option<ChangePatch> {
        let revision = repository.get_revision();
        let entries = repository.get_all_entries();
        let changes = entries.iter()
                .map(|d| FileChange::new((*d).clone(), ChangeType::DoDownload))
                .collect();

        Some(ChangePatch::new(revision, changes))
    }

    fn build_patch_for(rev: u64, client_list: &Vec<FileDefinition>, repository: &FileRepository) -> Option<ChangePatch> {
        let latest_rev = repository.get_revision();
        let server_ahead = latest_rev > rev;
        let mut res = Vec::new();
        let server_list = repository.get_all_entries();
        let mut srv_fds_map: HashMap<String, &FileDefinition> = server_list.iter()
                .map(|df| (df.id.as_ref().unwrap().clone(), *df))
                .collect();

        for client_fd in client_list {
            let client_fd_id = client_fd.id.as_ref().unwrap();
            if !srv_fds_map.contains_key(client_fd_id) {
                        // Client has file that server doesn't.
                    // TODO check in history between revs to consider remote deletion, instead of just server_ahead.
                    // This is very finicky and not a good solution in general.
                let was_deleted = server_ahead;
                if was_deleted {
                    res.push(FileChange::new(client_fd.clone(), ChangeType::Delete));
                }
                else {
                    res.push(FileChange::new(client_fd.clone(), ChangeType::Create));
                }
                continue;
            }

                    // Definitions exist in both client and server.
            let server_fd = *srv_fds_map.get(client_fd_id).unwrap();
            let file_is_same = Self::fuzzy_compare(client_fd, server_fd);
            if !file_is_same {
                    // TODO do better checking, probably with revisions
                if server_fd.last_update >= client_fd.last_update {
                        // Server file is more recent.
                    res.push(FileChange::new(server_fd.clone(), ChangeType::DoDownload));
                }
                else {
                    res.push(FileChange::new(client_fd.clone(), ChangeType::DoUpload));
                }
            }
            else {
                // noop
            }
            srv_fds_map.remove(client_fd_id);   // Remove matched file.
        }

            // Thanks to removal on match; files that exist in the server and not in the client
        for server_only_fs in srv_fds_map.values() {
            println!("Server only file: {:#?}", &server_only_fs);
                // Deleted files should be handled during local-patch, so all thhese should be new files.
            res.push(FileChange::new((*server_only_fs).clone(), ChangeType::DoDownload));
        }

        Some(ChangePatch::new(latest_rev, res))
    }

    fn fuzzy_compare(a: &FileDefinition, b: &FileDefinition) -> bool {
        a.name == b.name
                && a.path == b.path
                && a.size == b.size
                && a.checksum == b.checksum

    }
}
