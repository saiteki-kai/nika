use std::result;

use rusqlite::{params, Row};

use super::sqlite::Storage;
use crate::errors::NikaResult;
use crate::models::study::{DailyItem, DailyList, Status, StudyItemProgress};

impl Storage {
    pub fn get_daily_list(&self) -> NikaResult<DailyList> {
        let items = self
            .db
            .prepare(
                "SELECT s.word_id, s.status, s.created_at, s.updated_at, d.sort_index
                 FROM study_item_progress as s
                 JOIN daily_list as d ON s.word_id = d.word_id
                 ORDER BY d.sort_index",
            )?
            .query_map(params![], row_to_daily_item)?
            .collect::<Result<Vec<DailyItem>, _>>()?;

        Ok(DailyList::new(items))
    }

    pub fn import_daily_list(&self, list: DailyList) -> NikaResult<()> {
        self.db.execute_batch("BEGIN TRANSACTION;")?;
        self.db.execute_batch("DELETE FROM daily_list;")?;

        for item in list.items {
            self.insert_daily_item_with_progress(item, false)?;
        }

        self.db.execute_batch("COMMIT TRANSACTION;")?;

        Ok(())
    }

    pub fn insert_daily_item(&self, item: DailyItem) -> NikaResult<()> {
        self.insert_daily_item_with_progress(item, true)
    }

    pub fn update_daily_item_progress(&self, item: DailyItem) -> NikaResult<()> {
        self.db
            .prepare_cached("UPDATE study_item_progress SET status = ?2, updated_at = ?3, created_at = ?4 WHERE word_id = ?1")?
            .execute(params![item.word_id, item.progress.status, item.progress.updated_at, item.progress.created_at])?;

        Ok(())
    }

    fn insert_daily_item_with_progress(
        &self,
        item: DailyItem,
        use_transaction: bool,
    ) -> NikaResult<()> {
        if use_transaction {
            self.db.execute_batch("BEGIN TRANSACTION;")?;
        }

        self.db
            .prepare_cached("INSERT INTO daily_list (word_id, sort_index) VALUES (?1, ?2)")?
            .insert(params![item.word_id, item.sort_index])?;

        self.db
            .prepare_cached("INSERT OR IGNORE INTO study_item_progress (word_id, status, updated_at, created_at) VALUES (?1, ?2, ?3, ?4)")?
            .insert(params![item.word_id, item.progress.status, item.progress.updated_at, item.progress.created_at])?;

        if use_transaction {
            self.db.execute_batch("END TRANSACTION;")?;
        }

        Ok(())
    }
}

fn row_to_daily_item(row: &Row) -> result::Result<DailyItem, rusqlite::Error> {
    let item = DailyItem {
        word_id: row.get(0)?,
        progress: StudyItemProgress {
            status: row.get::<_, Status>(1)?,
            created_at: row.get::<_, i64>(2)?,
            updated_at: row.get::<_, i64>(3)?,
        },
        sort_index: row.get(4)?,
    };

    Ok(item)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_storage() -> Storage {
        Storage::open_in_memory().expect("failed to open storage")
    }

    #[test]
    fn test_get_empty_list() {
        let storage = setup_storage();
        let list = storage.get_daily_list().expect("Failed to get list");
        assert_eq!(list.items.len(), 0);
    }

    #[test]
    fn test_get_list() {
        let storage = setup_storage();

        let item1 = DailyItem {
            word_id: "abc".to_owned(),
            progress: StudyItemProgress::default(),
            sort_index: 5,
        };

        let item2 = DailyItem {
            word_id: "def".to_owned(),
            progress: StudyItemProgress::default(),
            sort_index: 3,
        };

        let item3 = DailyItem {
            word_id: "ghi".to_owned(),
            progress: StudyItemProgress::default(),
            sort_index: 7,
        };

        let list = DailyList::new(vec![item1.clone(), item2.clone(), item3.clone()]);

        storage
            .import_daily_list(list)
            .expect("Failed to import list");

        let list = storage.get_daily_list().expect("Failed to get list");
        assert_eq!(list.items.len(), 3);
        assert_eq!(list.items[0], item2);
        assert_eq!(list.items[1], item1);
        assert_eq!(list.items[2], item3);
    }

    #[test]
    fn test_insert_item() {
        let storage = setup_storage();

        let item = DailyItem {
            word_id: "1".to_owned(),
            progress: StudyItemProgress::default(),
            sort_index: 0,
        };

        storage
            .insert_daily_item(item)
            .expect("Failed to insert item");

        let list = storage.get_daily_list().expect("Failed to get list");
        assert_eq!(list.items.len(), 1);
    }

    #[test]
    fn test_import_list() {
        let storage = setup_storage();

        let item1 = DailyItem {
            word_id: "1".to_owned(),
            progress: StudyItemProgress::default(),
            sort_index: 0,
        };

        let item2 = DailyItem {
            word_id: "2".to_owned(),
            progress: StudyItemProgress::default(),
            sort_index: 1,
        };

        let list = DailyList::new(vec![item1.clone(), item2.clone()]);
        storage
            .import_daily_list(list)
            .expect("Failed to import list");

        let list = storage.get_daily_list().expect("Failed to get list");
        assert!(list.items == vec![item1, item2]);
    }

    #[test]
    fn test_update_progress() {
        let storage = setup_storage();
        let word_id = "42".to_owned();

        let item = DailyItem {
            word_id: word_id.clone(),
            progress: StudyItemProgress::default(),
            sort_index: 0,
        };

        storage
            .insert_daily_item(item.clone())
            .expect("Failed to insert item");

        let item = DailyItem {
            word_id: word_id.clone(),
            progress: StudyItemProgress {
                status: Status::Done,
                updated_at: 7777,
                created_at: 1234,
            },
            sort_index: 9,
        };

        storage
            .update_daily_item_progress(item.clone())
            .expect("Failed to update item");

        let list = storage.get_daily_list().expect("Failed to get list");
        assert_eq!(list.items.len(), 1);
        assert_eq!(list.items[0].word_id, word_id);
        assert_ne!(list.items[0].sort_index, item.sort_index);
        assert_eq!(list.items[0].progress.status, Status::Done);
        assert_eq!(list.items[0].progress.updated_at, 7777);
        assert_eq!(list.items[0].progress.created_at, 1234);
    }
}
