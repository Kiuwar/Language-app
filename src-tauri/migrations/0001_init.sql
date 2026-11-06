CREATE TABLE settings (
  key TEXT PRIMARY KEY,
  value TEXT NOT NULL
);

CREATE TABLE tags (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL UNIQUE
);

CREATE TABLE user_words (
  id INTEGER PRIMARY KEY,
  lemma TEXT NOT NULL,
  language_code TEXT NOT NULL DEFAULT 'en',
  dictionary_entry_id INTEGER,
  custom_translation TEXT,
  state TEXT NOT NULL CHECK (state IN ('nova', 'aprendendo', 'dominada')) DEFAULT 'nova',
  consecutive_correct INTEGER NOT NULL DEFAULT 0,
  ease_factor REAL NOT NULL DEFAULT 2.5,
  interval_days INTEGER NOT NULL DEFAULT 0,
  due_at TEXT,
  origin_phrase TEXT,
  origin_app TEXT,
  created_at TEXT NOT NULL,
  last_reviewed_at TEXT,
  UNIQUE (lemma, language_code)
);

CREATE TABLE word_tags (
  user_word_id INTEGER NOT NULL REFERENCES user_words (id) ON DELETE CASCADE,
  tag_id INTEGER NOT NULL REFERENCES tags (id) ON DELETE CASCADE,
  PRIMARY KEY (user_word_id, tag_id)
);

CREATE TABLE reviews (
  id INTEGER PRIMARY KEY,
  user_word_id INTEGER NOT NULL REFERENCES user_words (id) ON DELETE CASCADE,
  reviewed_at TEXT NOT NULL,
  grade TEXT NOT NULL CHECK (grade IN ('errou', 'dificil', 'bom', 'facil')),
  resulting_interval INTEGER NOT NULL,
  resulting_state TEXT NOT NULL
);

CREATE TABLE daily_activity (
  date TEXT PRIMARY KEY,
  reviews_count INTEGER NOT NULL DEFAULT 0,
  new_words_count INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX idx_user_words_due ON user_words (due_at) WHERE state != 'dominada';
CREATE INDEX idx_user_words_state ON user_words (state);
