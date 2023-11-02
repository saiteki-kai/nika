use darkbird::document::{Document, FullText, Indexer, MaterializedView, Range, RangeField, Tags};

use crate::core::models::dictionary::Word;

impl Document for Word {}

impl Indexer for Word {
    fn extract(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl Tags for Word {
    fn get_tags(&self) -> Vec<String> {
        vec![]
    }
}

impl Range for Word {
    fn get_fields(&self) -> Vec<RangeField> {
        vec![]
    }
}

impl MaterializedView for Word {
    fn filter(&self) -> Option<String> {
        None
    }
}

impl FullText for Word {
    fn get_content(&self) -> Option<String> {
        None
    }
}
