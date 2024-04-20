use super::study_item_progress::StudyItemProgress;
use super::Status;

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

    pub fn from(word_id: String, sort_index: i64, status: Status) -> Self {
        Self {
            word_id,
            progress: StudyItemProgress {
                status,
                ..Default::default()
            },
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

    pub fn from(word_id: String, created_at: i64, status: Status) -> Self {
        Self {
            word_id,
            progress: StudyItemProgress {
                status,
                ..Default::default()
            },
            created_at,
        }
    }
}

pub trait HasProgressStatus {
    fn status(&self) -> Status;
}

impl HasProgressStatus for DailyItem {
    fn status(&self) -> Status {
        self.progress.status
    }
}

impl HasProgressStatus for DiscoveryItem {
    fn status(&self) -> Status {
        self.progress.status
    }
}
