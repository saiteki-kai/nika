CREATE TABLE
    IF NOT EXISTS study_item_progress (
        word_id TEXT PRIMARY KEY, -- word id
        status TEXT NOT NULL, -- status of the word
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP, -- date when the word was added
        updated_at DATETIME DEFAULT CURRENT_TIMESTAMP -- date when the word was last updated
    );

CREATE TABLE
    IF NOT EXISTS daily_list (
        word_id TEXT PRIMARY KEY, -- word id
        sort_index INTEGER NOT NULL UNIQUE -- order of the word in the study list
    );

CREATE TABLE
    IF NOT EXISTS discovery_list (
        word_id TEXT PRIMARY KEY, -- word id
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP -- date when the word was added
    );

CREATE TABLE
    IF NOT EXISTS statistics (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        due INTEGER NOT NULL, -- total number of words due today
        done INTEGER NOT NULL, -- total number of words done today
        streak INTEGER NOT NULL, -- current streak
        date DATE DEFAULT CURRENT_DATE UNIQUE -- date of the study session
    );
