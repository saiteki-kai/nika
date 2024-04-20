use chrono::Utc;

use super::word_status::Status;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StudyItemProgress {
    pub status: Status,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Default for StudyItemProgress {
    fn default() -> Self {
        let now = Utc::now().timestamp_micros();

        Self {
            status: Status::New,
            created_at: now,
            updated_at: now,
        }
    }
}
