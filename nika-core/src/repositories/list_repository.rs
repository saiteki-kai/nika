use std::fs;
use std::fs::DirEntry;
use std::path::PathBuf;
use std::result;

use crate::errors::NikaError;
use crate::errors::Result;
use crate::errors::StudyListError;
use crate::models::study_list::StudyConfig;
use crate::models::study_list::StudyList;

/// `ListRepository` is a struct that represents a list repository.
pub struct ListRepository {
    /// `dirpath` is a `PathBuf` that specifies the path where the lists will be
    /// stored.
    dirpath: PathBuf,
}

impl ListRepository {
    /// `new` creates a new `ListRepository` instance with the given `dirpath`.
    pub fn new(dirpath: PathBuf) -> Self {
        Self { dirpath }
    }

    pub fn add_list(&self, study_list: StudyList) -> Result<()> {
        let filepath = self.list_filepath(&study_list.name);

        if filepath.exists() {
            return Err(NikaError::List(StudyListError::ListAlreadyExists));
        }

        StudyList::save(&filepath, study_list)
    }

    pub fn remove_list(&self, name: &str) -> Result<()> {
        let filepath = self.list_filepath(name);

        if !filepath.exists() {
            return Err(NikaError::List(StudyListError::ListNotFound));
        }

        fs::remove_file(filepath)?;
        Ok(())
    }

    pub fn get_lists(&self) -> Result<Vec<StudyList>> {
        let entries = fs::read_dir(&self.dirpath)?.collect::<Vec<result::Result<DirEntry, _>>>();

        let mut lists: Vec<StudyList> = Vec::with_capacity(entries.len());

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                lists.push(StudyList::load(&path)?);
            }
        }

        Ok(lists)
    }

    pub fn get_list(&self, name: &str) -> Result<StudyList> {
        let filepath = self.list_filepath(name);

        if !filepath.exists() {
            return Err(NikaError::List(StudyListError::ListNotFound));
        }

        StudyList::load(&filepath)
    }

    pub fn update_list_config(&self, name: &str, config: StudyConfig) -> Result<()> {
        let filepath = self.list_filepath(name);

        if !filepath.exists() {
            return Err(NikaError::List(StudyListError::ListNotFound));
        }

        let study_list = StudyList::load(&filepath)?;

        StudyList::save(
            &filepath,
            StudyList {
                config,
                ..study_list
            },
        )
    }

    fn list_filepath(&self, name: &str) -> PathBuf {
        self.dirpath.join(format!("{}.bin", name))
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    fn setup() -> (PathBuf, ListRepository) {
        let tmp_path = tempdir().unwrap().into_path();
        let list_repo = ListRepository::new(tmp_path.clone());

        (tmp_path, list_repo)
    }

    #[test]
    fn test_initialization() {
        let (tmp_path, list_repo) = setup();

        assert_eq!(tmp_path, list_repo.dirpath);

        let lists = list_repo.get_lists().unwrap();
        assert!(lists.is_empty());
    }

    #[test]
    fn test_list_filepath() {
        let (tmp_path, list_repo) = setup();

        let name = "list1";
        let filepath = list_repo.list_filepath(name);

        assert_eq!(filepath, tmp_path.join(format!("{}.bin", name)));
    }
}
