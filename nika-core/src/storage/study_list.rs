use std::result;

use rusqlite::params;
use rusqlite::Row;

use super::sqlite::Storage;
use crate::errors::Result;
use crate::models::study_list::Status;
use crate::models::study_list::StudyItem;
use crate::models::study_list::StudyList;

fn row_to_study_item(row: &Row) -> result::Result<StudyItem, rusqlite::Error> {
    let list = StudyItem {
        word_id: row.get(0)?,
        status: row.get::<_, Status>(1)?,
        updated_at: row.get::<_, i64>(2)?,
    };

    Ok(list)
}

impl Storage {
    pub fn get_study_list(&self) -> Result<StudyList> {
        let items = self
            .db
            .prepare("SELECT word_id, status, updated_at FROM study_list")?
            .query_map(params![], row_to_study_item)?
            .collect::<Result<Vec<StudyItem>, _>>()?;

        Ok(StudyList::new(items))
    }

    // pub fn insert_study_list(&self, list: StudyList) -> Result<()> {
    //     let mut tx = self.db.transaction()?;
    //     Self::insert_many(&mut tx, list.items)?;
    //     tx.commit()?;

    //     Ok(())
    // }

    pub fn insert_study_item(&self, item: StudyItem) -> Result<()> {
        let mut stmt = self.db.prepare_cached(
            "INSERT INTO study_list (word_id, status, updated_at) VALUES (?1, ?2, ?3)",
        )?;
        stmt.insert(params![item.word_id, item.status, item.updated_at])?;

        Ok(())
    }

    pub fn update_study_item(&self, item: StudyItem) -> Result<()> {
        self.db
            .prepare("UPDATE study_list SET status = ?2, updated_at = ?3 WHERE word_id = ?1")?
            .execute(params![item.word_id, item.status, item.updated_at])?;

        Ok(())
    }

    // fn insert_many(tx: &mut Transaction, items: Vec<StudyItem>) -> Result<()> {
    //     let mut stmt = tx.prepare_cached(
    //         "INSERT INTO study_list (word_id, status, updated_at) VALUES (?1, ?2,
    // ?3)",     )?;

    //     for item in items {
    //         stmt.insert(params![item.word_id, item.status, item.updated_at])?;
    //     }

    //     Ok(())
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::study_list::Status::Done;
    use crate::models::study_list::Status::New;

    #[test]
    fn test() {
        let storage = Storage::open_in_memory().expect("failed to open storage");

        let item1 = StudyItem {
            word_id: "1".to_string(),
            status: New,
            updated_at: 0,
        };

        let item2 = StudyItem {
            word_id: "2".to_string(),
            status: Done,
            updated_at: 1,
        };

        let list = StudyList::new(vec![item1.clone(), item2.clone()]);

        for item in list.items {
            let result = storage.insert_study_item(item);
            assert!(result.is_ok());
        }
        // let result = storage.insert_study_list(list);
        // assert!(result.is_ok());

        let result = storage.get_study_list();
        assert!(result.is_ok());

        let list = result.unwrap();
        assert_eq!(list.items.len(), 2);
        assert_eq!(list.items[0], item1);
        assert_eq!(list.items[1], item2);
    }
}
