use super::study_item::DailyItem;
use super::study_item::DiscoveryItem;
use super::study_item::Status;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudyList<T> {
    pub items: Vec<T>,
}

impl<T: HasProgressStatus> StudyList<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self { items }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn filter_by_status(&self, status: Status) -> impl Iterator<Item = &T> {
        self.items
            .iter()
            .filter(move |item| item.status() == status)
    }

    pub fn count(&self, status: Status) -> usize {
        self.filter_by_status(status).count()
    }
}

pub type DailyList = StudyList<DailyItem>;
pub type DiscoveryList = StudyList<DiscoveryItem>;
