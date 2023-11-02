use darkbird::dashmap::mapref::one::Ref;
use darkbird::{Database, Options, Schema, SessionResult, StorageType};
use std::sync::atomic::AtomicU64;

use crate::core::models::dictionary::Word;

pub struct WordRepository {
    db: Database,
    seq: AtomicU64,
}

impl WordRepository {
    pub async fn new(path: &str, name: &str) -> Self {
        let opts = Options::new(path, name, 1000, StorageType::DiskCopies, true);

        WordRepository {
            seq: AtomicU64::new(1),
            db: Schema::new()
                .with_datastore::<u64, Word>(opts)
                .await
                .unwrap()
                .build(),
        }
    }

    pub fn get_by_index(&self, key: &str) -> Result<Option<Ref<u64, Word>>, SessionResult> {
        self.db.lookup_by_index::<u64, Word>(key)
    }

    pub async fn insert(&self, word: Word) -> Result<(), SessionResult> {
        let id = self.seq.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.db.insert::<u64, Word>(id, word).await
    }
}
