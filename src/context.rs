#![allow(dead_code)]
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;
use directories::ProjectDirs;
use lazycell::LazyCell;
use nika_core::dictionary::Dictionary;
use nika_core::dictionary::TagMap;
use nika_core::dictionary::WordMap;
use nika_core::preferences::UserPreferences;
use nika_core::storage::sqlite::Storage;

pub struct GlobalContext {
    dirs: ProjectDirs,
    prefs: LazyCell<UserPreferences>,
    dictionary: LazyCell<Dictionary>,
    db: LazyCell<Storage>,
}

impl GlobalContext {
    pub fn new(dirs: ProjectDirs) -> Result<Self> {
        Ok(Self {
            dirs,
            prefs: LazyCell::new(),
            dictionary: LazyCell::new(),
            db: LazyCell::new(),
        })
    }

    pub fn db(&self) -> Result<&Storage> {
        let db = self.db.try_borrow_with(|| Storage::open(&self.db_path()))?;

        Ok(db)
    }

    pub fn db_mut(&mut self) -> Result<&mut Storage> {
        let _ = self.db()?;
        let db = self.db.borrow_mut().expect("db should be initialized");

        Ok(db)
    }

    pub fn dictionary(&self) -> Result<&Dictionary> {
        let dictionary = self.dictionary.try_borrow_with(|| {
            let contents = fs::read(self.words_bin_path())?;
            let words = bincode::deserialize::<WordMap>(&contents)?;

            let contents = fs::read(self.tags_bin_path())?;
            let tags = bincode::deserialize::<TagMap>(&contents)?;

            Ok::<_, anyhow::Error>(Dictionary::from(words, tags))
        })?;

        Ok(dictionary)
    }

    pub fn prefs(&self) -> Result<&UserPreferences> {
        let prefs = self
            .prefs
            .try_borrow_with(|| UserPreferences::load(&self.prefs_path()))?;

        Ok(prefs)
    }

    fn data_path(&self) -> &Path {
        self.dirs.data_dir()
    }

    fn config_path(&self) -> &Path {
        self.dirs.config_dir()
    }

    fn cache_path(&self) -> &Path {
        self.dirs.cache_dir()
    }

    fn db_path(&self) -> PathBuf {
        self.data_path().join("nika.db")
    }

    fn prefs_path(&self) -> PathBuf {
        self.config_path().join("preferences.json")
    }

    fn words_bin_path(&self) -> PathBuf {
        self.data_path().join("jmdict-words.bin")
    }

    fn tags_bin_path(&self) -> PathBuf {
        self.data_path().join("jmdict-tags.bin")
    }

    fn kanji_bin_path(&self) -> PathBuf {
        self.data_path().join("kanjidic.bin")
    }
}
