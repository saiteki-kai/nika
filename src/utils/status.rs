use clap::ValueEnum;
use nika_core::models::study_item::Status;

#[derive(ValueEnum, Clone)]
pub enum WordStatus {
    New,
    Skipped,
    Discarded,
    Done,
}

impl From<&Status> for WordStatus {
    fn from(value: &Status) -> Self {
        match value {
            Status::New => WordStatus::New,
            Status::Skipped => WordStatus::Skipped,
            Status::Discarded => WordStatus::Discarded,
            Status::Done => WordStatus::Done,
        }
    }
}

impl From<&WordStatus> for Status {
    fn from(value: &WordStatus) -> Self {
        match value {
            WordStatus::New => Status::New,
            WordStatus::Skipped => Status::Skipped,
            WordStatus::Discarded => Status::Discarded,
            WordStatus::Done => Status::Done,
        }
    }
}
