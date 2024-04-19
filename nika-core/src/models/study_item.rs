use chrono::DateTime;
use chrono::Utc;
use rusqlite::types::FromSql;
use rusqlite::types::FromSqlResult;
use rusqlite::types::ToSqlOutput;
use rusqlite::types::ValueRef;
use rusqlite::ToSql;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Status {
    Skipped,
    Discarded,
    Done,
    New,
}

impl From<&Status> for String {
    fn from(value: &Status) -> Self {
        match &value {
            Status::Skipped => "skipped".into(),
            Status::Discarded => "discarded".into(),
            Status::Done => "done".into(),
            Status::New => "todo".into(),
        }
    }
}

impl From<&str> for Status {
    fn from(value: &str) -> Self {
        match value {
            "skipped" => Status::Skipped,
            "discarded" => Status::Discarded,
            "done" => Status::Done,
            "todo" => Status::New,
            _ => Status::New,
        }
    }
}

impl From<&Status> for &[u8] {
    fn from(val: &Status) -> Self {
        match val {
            Status::Skipped => b"skipped",
            Status::Discarded => b"discarded",
            Status::Done => b"done",
            Status::New => b"todo",
        }
    }
}

impl ToSql for Status {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let value = ValueRef::Text(self.into());
        Ok(ToSqlOutput::Borrowed(value))
    }
}

impl FromSql for Status {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        Ok(Status::from(value.as_str()?))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

pub fn date(timestamp: i64) -> Option<DateTime<Utc>> {
    DateTime::from_timestamp_micros(timestamp)
}
