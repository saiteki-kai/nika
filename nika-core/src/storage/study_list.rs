use super::sqlite::Storage;
use crate::errors::NikaResult;
use crate::models::study::DailyItem;

impl Storage {
    pub fn get_study_list(&self) -> NikaResult<Vec<DailyItem>> {
        todo!()
    }
}
