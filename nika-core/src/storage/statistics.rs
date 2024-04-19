use rusqlite::params;
use rusqlite::Row;

use super::sqlite::Storage;
use crate::errors::NikaResult;
use crate::models::statistics::Statistics;

fn row_to_statistics(row: &Row) -> rusqlite::Result<Statistics, rusqlite::Error> {
    let statistics = Statistics {
        streak: row.get(0)?,
        done: row.get(1)?,
        due: row.get(2)?,
        date: row.get(3)?,
    };

    Ok(statistics)
}

impl Storage {
    pub fn get_all_statistics(&mut self) -> NikaResult<Vec<Statistics>> {
        let rows = self
            .db
            .prepare("SELECT streak, done, due, date FROM statistics")?
            .query_map(params![], row_to_statistics)?
            .collect::<NikaResult<Vec<Statistics>, _>>()?;

        Ok(rows)
    }

    pub fn get_statistics_by_date(&mut self, timestamp: String) -> NikaResult<Statistics> {
        let row = self
            .db
            .prepare("SELECT streak, done, due, date FROM statistics WHERE date = ?1")?
            .query_row(params![timestamp], row_to_statistics)?;

        Ok(row)
    }

    pub fn update_statistics(&mut self, stats: Statistics) -> NikaResult<()> {
        // TODO: filter by date / id

        self.db
            .prepare("INSERT OR REPLACE INTO statistics (date, streak, done, due) VALUES (?1, ?2, ?3, ?4)")?
            .execute(params![stats.date, stats.streak, stats.done, stats.due])?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_statistics() {
        let mut storage = Storage::open_in_memory().expect("failed to open storage");

        let result = storage.get_all_statistics();
        assert!(result.is_ok());

        let stats = result.unwrap();
        assert!(stats.is_empty());
    }

    #[test]
    fn test_get_statistics_by_date() {
        let mut storage = Storage::open_in_memory().expect("failed to open storage");

        let stats = Statistics::new(10, 2, 5, "2024-04-04".to_owned());

        storage.update_statistics(stats).unwrap();

        let result = storage.get_statistics_by_date("2024-04-04".to_owned());
        assert!(result.is_ok());

        let stats = result.unwrap();
        assert_eq!(stats.streak, 10);
        assert_eq!(stats.done, 2);
        assert_eq!(stats.due, 5);
        assert_eq!(stats.date, "2024-04-04".to_owned());
    }
}
