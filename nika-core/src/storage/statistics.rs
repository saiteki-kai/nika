use rusqlite::{params, Row};

use super::sqlite::Storage;
use crate::errors::NikaResult;
use crate::models::study::StudyStatistics;

fn row_to_statistics(row: &Row) -> rusqlite::Result<StudyStatistics, rusqlite::Error> {
    let statistics = StudyStatistics {
        streak: row.get(0)?,
        done: row.get(1)?,
        due: row.get(2)?,
        date: row.get(3)?,
    };

    Ok(statistics)
}

impl Storage {
    pub fn get_study_statistics(&mut self) -> NikaResult<Vec<StudyStatistics>> {
        let rows = self
            .db
            .prepare("SELECT streak, done, due, date FROM study_statistics")?
            .query_map(params![], row_to_statistics)?
            .collect::<NikaResult<Vec<StudyStatistics>, _>>()?;

        Ok(rows)
    }

    pub fn get_study_statistics_by_date(
        &mut self,
        timestamp: String,
    ) -> NikaResult<StudyStatistics> {
        let row = self
            .db
            .prepare("SELECT streak, done, due, date FROM study_statistics WHERE date = ?1")?
            .query_row(params![timestamp], row_to_statistics)?;

        Ok(row)
    }

    pub fn update_study_statistics(&mut self, stats: StudyStatistics) -> NikaResult<()> {
        // TODO: filter by date / id

        self.db
            .prepare("INSERT OR REPLACE INTO study_statistics (date, streak, done, due) VALUES (?1, ?2, ?3, ?4)")?
            .execute(params![stats.date, stats.streak, stats.done, stats.due])?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_study_statistics() {
        let mut storage = Storage::open_in_memory().expect("failed to open storage");

        let result = storage.get_study_statistics();
        assert!(result.is_ok());

        let stats = result.unwrap();
        assert!(stats.is_empty());
    }

    #[test]
    fn test_get_statistics_by_date() {
        let mut storage = Storage::open_in_memory().expect("failed to open storage");

        let stats = StudyStatistics::new(10, 2, 5, "2024-04-04".to_owned());

        storage.update_study_statistics(stats).unwrap();

        let result = storage.get_study_statistics_by_date("2024-04-04".to_owned());
        assert!(result.is_ok());

        let stats = result.unwrap();
        assert_eq!(stats.streak, 10);
        assert_eq!(stats.done, 2);
        assert_eq!(stats.due, 5);
        assert_eq!(stats.date, "2024-04-04".to_owned());
    }
}
