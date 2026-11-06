use rusqlite::{Connection, OptionalExtension};
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct DictionaryEntry {
    pub lemma: String,
    pub pos: Option<String>,
    pub ipa: Option<String>,
    pub translation_pt: String,
    pub examples: Vec<String>,
}

fn row_to_entry(row: &rusqlite::Row) -> rusqlite::Result<DictionaryEntry> {
    let examples_json: String = row.get(4)?;
    let examples: Vec<String> = serde_json::from_str(&examples_json).unwrap_or_default();
    Ok(DictionaryEntry {
        lemma: row.get(0)?,
        pos: row.get(1)?,
        ipa: row.get(2)?,
        translation_pt: row.get(3)?,
        examples,
    })
}

pub fn lookup_entry(
    conn: &Connection,
    lemma: &str,
    language_code: &str,
) -> Option<DictionaryEntry> {
    conn.query_row(
        "SELECT lemma, pos, ipa, translation_pt, examples_json
         FROM dict.dictionary_entries
         WHERE lemma = ?1 AND language_code = ?2
         LIMIT 1",
        (lemma, language_code),
        row_to_entry,
    )
    .optional()
    .unwrap_or(None)
}

pub fn search_entries(conn: &Connection, query: &str, limit: i64) -> Vec<DictionaryEntry> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        return Vec::new();
    }
    let fts_query = format!("{trimmed}*");

    // O operando esquerdo do MATCH tem que ser o nome literal da tabela
    // FTS5 (nem alias, nem schema.tabela), mesmo com ela anexada via
    // ATTACH — é assim que o SQLite reconhece a sintaxe especial do FTS5.
    let mut stmt = match conn.prepare(
        "SELECT e.lemma, e.pos, e.ipa, e.translation_pt, e.examples_json
         FROM dict.dictionary_fts f
         JOIN dict.dictionary_entries e ON e.id = f.rowid
         WHERE dictionary_fts MATCH ?1
         LIMIT ?2",
    ) {
        Ok(stmt) => stmt,
        Err(err) => {
            eprintln!("busca no dicionário falhou (prepare): {err}");
            return Vec::new();
        }
    };

    let rows = stmt.query_map((&fts_query, limit), row_to_entry);

    match rows {
        Ok(rows) => rows.filter_map(Result::ok).collect(),
        Err(err) => {
            eprintln!("busca no dicionário falhou (query): {err}");
            Vec::new()
        }
    }
}

pub fn is_known(conn: &Connection, lemma: &str, language_code: &str) -> bool {
    conn.query_row(
        "SELECT 1 FROM user_words WHERE lemma = ?1 AND language_code = ?2 AND state = 'dominada'",
        (lemma, language_code),
        |_| Ok(()),
    )
    .optional()
    .unwrap_or(None)
    .is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dictionary::test_support::test_connection;

    #[test]
    fn lookup_entry_finds_seed_word() {
        let conn = test_connection();
        let entry = lookup_entry(&conn, "run", "en").expect("entrada esperada");
        assert_eq!(entry.translation_pt, "correr");
        assert_eq!(entry.pos.as_deref(), Some("verbo"));
        assert_eq!(entry.examples.len(), 2);
    }

    #[test]
    fn lookup_entry_returns_none_for_unknown_word() {
        let conn = test_connection();
        assert!(lookup_entry(&conn, "xyzxyz", "en").is_none());
    }

    #[test]
    fn search_entries_matches_prefix() {
        let conn = test_connection();
        let results = search_entries(&conn, "run", 10);
        assert!(results.iter().any(|entry| entry.lemma == "run"));
    }

    #[test]
    fn search_entries_empty_query_returns_nothing() {
        let conn = test_connection();
        assert!(search_entries(&conn, "   ", 10).is_empty());
    }

    #[test]
    fn is_known_false_when_not_saved() {
        let conn = test_connection();
        assert!(!is_known(&conn, "run", "en"));
    }

    #[test]
    fn is_known_true_when_mastered() {
        let conn = test_connection();
        conn.execute(
            "INSERT INTO user_words (lemma, language_code, state) VALUES ('run', 'en', 'dominada')",
            [],
        )
        .unwrap();
        assert!(is_known(&conn, "run", "en"));
    }
}
