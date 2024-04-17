use chrono::DateTime;
use chrono::Utc;
use rusqlite::types::FromSql;
use rusqlite::types::FromSqlResult;
use rusqlite::types::ToSqlOutput;
use rusqlite::types::ValueRef;
use rusqlite::ToSql;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Debug)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct StudyItem {
    pub word_id: String,
    pub status: Status,
    pub updated_at: i64,
}

impl StudyItem {
    pub fn new(word_id: String, status: Status, updated_at: i64) -> Self {
        Self {
            word_id,
            status,
            updated_at,
        }
    }

    pub fn from(word_id: String) -> Self {
        Self::new(word_id, Status::New, Utc::now().timestamp_micros())
    }

    pub fn date(&self) -> Option<DateTime<Utc>> {
        DateTime::from_timestamp_micros(self.updated_at)
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct StudyList {
    // pub name: String,
    pub items: Vec<StudyItem>,
}

impl StudyList {
    pub fn new(items: Vec<StudyItem>) -> Self {
        Self { items }
    }

    pub fn total(&self) -> usize {
        self.items.len()
    }

    pub fn done(&self) -> usize {
        self.count(Status::Done)
    }

    pub fn todo(&self) -> usize {
        self.count(Status::New)
    }

    pub fn skipped(&self) -> usize {
        self.count(Status::Skipped)
    }

    pub fn discarded(&self) -> usize {
        self.count(Status::Discarded)
    }

    fn count(&self, status: Status) -> usize {
        self.items
            .iter()
            .filter(|item| item.status == status)
            .count()
    }
}
