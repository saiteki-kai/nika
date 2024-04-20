#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudyStatistics {
    pub streak: u32,
    pub done: u32,
    pub due: u32,
    pub date: String,
}

impl StudyStatistics {
    pub fn new(streak: u32, done: u32, due: u32, date: String) -> Self {
        Self {
            streak,
            done,
            due,
            date,
        }
    }
}
