use rusqlite::types::{FromSql, FromSqlResult, ToSqlOutput, ValueRef};
use rusqlite::{Result, ToSql};

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
            Status::New => "new".into(),
        }
    }
}

impl From<&str> for Status {
    fn from(value: &str) -> Self {
        match value {
            "skipped" => Status::Skipped,
            "discarded" => Status::Discarded,
            "done" => Status::Done,
            "new" => Status::New,
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
            Status::New => b"new",
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

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case("skipped", Status::Skipped)]
    #[test_case("discarded", Status::Discarded)]
    #[test_case("done", Status::Done)]
    #[test_case("new", Status::New)]
    fn test_from_str_valid(value: &str, status: Status) {
        assert_eq!(Status::from(value), status);
    }

    #[test_case("invalid")]
    #[test_case("")]
    fn test_from_str_invalid(value: &str) {
        assert_eq!(Status::from(value), Status::New);
    }

    #[test_case(Status::Skipped, "skipped".to_owned())]
    #[test_case(Status::Discarded, "discarded".to_owned())]
    #[test_case(Status::Done, "done".to_owned())]
    #[test_case(Status::New, "new".to_owned())]
    fn test_from_status(status: Status, value: String) {
        assert_eq!(String::from(&status), value);
    }

    #[test_case(Status::Skipped, b"skipped")]
    #[test_case(Status::Discarded, b"discarded")]
    #[test_case(Status::Done, b"done")]
    #[test_case(Status::New, b"new")]
    fn test_to_sql(status: Status, value: &[u8]) {
        assert_eq!(
            status.to_sql(),
            Ok(ToSqlOutput::Borrowed(ValueRef::Text(value)))
        );
    }

    #[test_case(b"skipped", Status::Skipped)]
    fn test_from_sql(value: &[u8], status: Status) {
        let value = ValueRef::Text(value);
        assert_eq!(Status::column_result(value), Ok(status));
    }

    #[test]
    fn test_from_sql_invalid() {
        let value = ValueRef::Text(b"invalid");
        assert_eq!(Status::column_result(value), Ok(Status::New));
    }
}
