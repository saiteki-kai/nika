use super::sqlite::Storage;
use crate::errors::NikaResult;
use crate::models::study_item::DailyItem;

impl Storage {
    pub fn get_study_list(&self) -> NikaResult<Vec<DailyItem>> {
        todo!()
    }
}
