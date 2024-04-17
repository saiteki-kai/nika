CREATE TABLE
    IF NOT EXISTS study_list (
        word_id TEXT PRIMARY KEY, -- NOT NULL UNIQUE,
        status TEXT NOT NULL,
        updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
    );

CREATE TABLE
    IF NOT EXISTS statistics (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        due INTEGER NOT NULL, -- total number of words due today
        done INTEGER NOT NULL, -- total number of words done today
        streak INTEGER NOT NULL, -- current streak
        date DATE DEFAULT CURRENT_DATE UNIQUE -- date of the study session
    );
