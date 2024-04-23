use super::study_item::{DailyItem, DiscoveryItem, HasProgressStatus};
use super::word_status::Status;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_empty() {
        let list = DailyList::new(vec![]);
        assert!(list.is_empty());

        let list = DiscoveryList::new(vec![]);
        assert!(list.is_empty());
    }

    #[test]
    fn test_len() {
        let mut items = vec![DailyItem::new("a".to_owned(), 0)];
        let list = StudyList::new(items.clone());
        assert_eq!(list.len(), 1);

        items.push(DailyItem::new("b".to_owned(), 1));
        let list = StudyList::new(items.clone());
        assert_eq!(list.len(), 2);

        items.push(DailyItem::new("c".to_owned(), 2));
        let list = StudyList::new(items.clone());
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn test_items() {
        let items = vec![
            DailyItem::new("a".to_owned(), 0),
            DailyItem::new("b".to_owned(), 1),
            DailyItem::new("c".to_owned(), 2),
        ];
        let list = StudyList::new(items.clone());

        for (i, item) in list.items.iter().enumerate() {
            assert_eq!(item, &items[i]);
            assert_eq!(item.sort_index, items[i].sort_index);
        }
    }

    #[test]
    fn test_status() {
        let item = DailyItem::from("a".to_owned(), 0, Status::Skipped);
        assert_eq!(item.status(), Status::Skipped);

        let item = DiscoveryItem::from("b".to_owned(), 1713640666825, Status::Discarded);
        assert_eq!(item.status(), Status::Discarded);
    }

    #[test]
    fn test_filter_by_status() {
        // 0 items
        let items = vec![];
        let list = DailyList::new(items);

        assert_eq!(list.filter_by_status(Status::New).count(), 0);
        assert_eq!(list.filter_by_status(Status::Skipped).count(), 0);
        assert_eq!(list.filter_by_status(Status::Discarded).count(), 0);
        assert_eq!(list.filter_by_status(Status::Done).count(), 0);

        // 1 item
        let items = vec![DailyItem::from("a".to_owned(), 0, Status::Skipped)];
        let list = StudyList::new(items);

        assert_eq!(list.filter_by_status(Status::New).count(), 0);
        assert_eq!(list.filter_by_status(Status::Skipped).count(), 1);
        assert_eq!(list.filter_by_status(Status::Discarded).count(), 0);
        assert_eq!(list.filter_by_status(Status::Done).count(), 0);

        // 8 items
        let items = vec![
            DailyItem::from("a".to_owned(), 0, Status::Skipped),
            DailyItem::from("b".to_owned(), 1, Status::Done),
            DailyItem::from("c".to_owned(), 2, Status::Discarded),
            DailyItem::from("d".to_owned(), 3, Status::New),
            DailyItem::from("e".to_owned(), 4, Status::New),
            DailyItem::from("f".to_owned(), 5, Status::New),
            DailyItem::from("g".to_owned(), 6, Status::Skipped),
            DailyItem::from("h".to_owned(), 7, Status::Done),
        ];
        let list = StudyList::new(items);

        assert_eq!(list.filter_by_status(Status::New).count(), 3);
        assert_eq!(list.filter_by_status(Status::Skipped).count(), 2);
        assert_eq!(list.filter_by_status(Status::Discarded).count(), 1);
        assert_eq!(list.filter_by_status(Status::Done).count(), 2);
    }

    #[test]
    fn test_count() {
        let items = vec![
            DiscoveryItem::new("a".to_owned(), 1713640328110),
            DiscoveryItem::new("b".to_owned(), 1713640328110),
            DiscoveryItem::new("c".to_owned(), 1713640328110),
            DiscoveryItem::new("d".to_owned(), 1713640328110),
        ];
        let list = StudyList::new(items);

        assert_eq!(list.count(Status::New), 4);
        assert_eq!(list.count(Status::Skipped), 0);
        assert_eq!(list.count(Status::Discarded), 0);
        assert_eq!(list.count(Status::Done), 0);
    }
}
