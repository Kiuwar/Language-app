#!/usr/bin/env python3
"""Gera data/dict/dictionary.db a partir de tools/build-dictionary/words.json.

Isto e um dicionario SEMENTE (curado a mao, poucas dezenas de palavras),
usado para validar o pipeline de ponta a ponta. A ingestao completa do
Wiktionary e um passo futuro separado (ver README.md desta pasta).
"""
import json
import pathlib
import sqlite3

HERE = pathlib.Path(__file__).parent
DB_PATH = HERE.parent.parent / "data" / "dict" / "dictionary.db"
WORDS_PATH = HERE / "words.json"

SCHEMA = """
CREATE TABLE dictionary_entries (
  id INTEGER PRIMARY KEY,
  lemma TEXT NOT NULL,
  language_code TEXT NOT NULL,
  pos TEXT,
  ipa TEXT,
  translation_pt TEXT NOT NULL,
  examples_json TEXT,
  pack TEXT NOT NULL DEFAULT 'base',
  source TEXT NOT NULL DEFAULT 'seed'
);

CREATE TABLE word_forms (
  surface_form TEXT NOT NULL,
  lemma TEXT NOT NULL,
  language_code TEXT NOT NULL,
  PRIMARY KEY (surface_form, language_code)
);

CREATE VIRTUAL TABLE dictionary_fts USING fts5(
  lemma, translation_pt, content='dictionary_entries', content_rowid='id'
);

CREATE INDEX idx_dictionary_entries_lemma ON dictionary_entries (lemma, language_code);
"""


def main() -> None:
    DB_PATH.parent.mkdir(parents=True, exist_ok=True)
    DB_PATH.unlink(missing_ok=True)

    words = json.loads(WORDS_PATH.read_text(encoding="utf-8"))

    conn = sqlite3.connect(DB_PATH)
    conn.executescript(SCHEMA)

    for word in words:
        cur = conn.execute(
            """INSERT INTO dictionary_entries
               (lemma, language_code, pos, ipa, translation_pt, examples_json, pack, source)
               VALUES (?, 'en', ?, ?, ?, ?, 'base', 'seed')""",
            (
                word["lemma"],
                word.get("pos"),
                word.get("ipa"),
                word["translation_pt"],
                json.dumps(word.get("examples", []), ensure_ascii=False),
            ),
        )
        entry_id = cur.lastrowid
        conn.execute(
            "INSERT INTO dictionary_fts (rowid, lemma, translation_pt) VALUES (?, ?, ?)",
            (entry_id, word["lemma"], word["translation_pt"]),
        )
        conn.execute(
            "INSERT OR IGNORE INTO word_forms (surface_form, lemma, language_code) VALUES (?, ?, 'en')",
            (word["lemma"], word["lemma"]),
        )
        for form in word.get("forms", []):
            conn.execute(
                "INSERT OR IGNORE INTO word_forms (surface_form, lemma, language_code) VALUES (?, ?, 'en')",
                (form, word["lemma"]),
            )

    conn.commit()
    entry_count = conn.execute("SELECT COUNT(*) FROM dictionary_entries").fetchone()[0]
    form_count = conn.execute("SELECT COUNT(*) FROM word_forms").fetchone()[0]
    conn.close()
    print(f"dictionary.db gerado: {entry_count} entradas, {form_count} formas.")


if __name__ == "__main__":
    main()
