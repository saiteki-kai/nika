use rusqlite::types::FromSql;
use rusqlite::types::FromSqlResult;
use rusqlite::types::ToSqlOutput;
use rusqlite::types::ValueRef;
use rusqlite::Result;
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
    fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
        let value = ValueRef::Text(self.into());
        Ok(ToSqlOutput::Borrowed(value))
    }
}

impl FromSql for Status {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        Ok(Status::from(value.as_str()?))
    }
}
