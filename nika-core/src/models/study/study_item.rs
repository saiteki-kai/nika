use super::study_item_progress::StudyItemProgress;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DailyItem {
    pub word_id: String,
    pub progress: StudyItemProgress,
    pub sort_index: i64,
}

impl DailyItem {
    pub fn new(word_id: String, sort_index: i64) -> Self {
        Self {
            word_id,
            progress: StudyItemProgress::default(),
            sort_index,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscoveryItem {
    pub word_id: String,
    pub progress: StudyItemProgress,
    pub created_at: i64,
}

impl DiscoveryItem {
    pub fn new(word_id: String, created_at: i64) -> Self {
        Self {
            word_id,
            progress: StudyItemProgress::default(),
            created_at,
        }
    }
}
