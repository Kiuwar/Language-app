use rusqlite::{Connection, OptionalExtension};

/// Resolve a forma base (lema) de uma palavra. Primeiro tenta a tabela
/// `word_forms` do dicionário (pré-computada); se não achar, cai para uma
/// heurística simples de sufixo, só para a busca nunca voltar vazia.
pub fn lemmatize(conn: &Connection, surface_form: &str, language_code: &str) -> String {
    let normalized = surface_form.trim().to_lowercase();
    if normalized.is_empty() {
        return normalized;
    }

    let found: Option<String> = conn
        .query_row(
            "SELECT lemma FROM dict.word_forms WHERE surface_form = ?1 AND language_code = ?2",
            (&normalized, language_code),
            |row| row.get(0),
        )
        .optional()
        .unwrap_or(None);

    found.unwrap_or_else(|| heuristic_lemma(&normalized))
}

fn heuristic_lemma(word: &str) -> String {
    if let Some(stem) = word.strip_suffix("ies") {
        return format!("{stem}y");
    }
    if word.ends_with("es") && word.len() > 3 {
        return word[..word.len() - 2].to_string();
    }
    if let Some(stem) = word.strip_suffix("ing") {
        if !stem.is_empty() {
            return stem.to_string();
        }
    }
    if let Some(stem) = word.strip_suffix("ed") {
        if !stem.is_empty() {
            return stem.to_string();
        }
    }
    if let Some(stem) = word.strip_suffix('s') {
        if !stem.is_empty() {
            return stem.to_string();
        }
    }
    word.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dictionary::test_support::test_connection;

    #[test]
    fn resolves_known_irregular_inflection() {
        let conn = test_connection();
        assert_eq!(lemmatize(&conn, "ran", "en"), "run");
        assert_eq!(lemmatize(&conn, "running", "en"), "run");
        assert_eq!(lemmatize(&conn, "RUNS", "en"), "run");
    }

    #[test]
    fn falls_back_to_heuristic_for_word_outside_dictionary() {
        let conn = test_connection();
        assert_eq!(lemmatize(&conn, "bananas", "en"), "banana");
        assert_eq!(lemmatize(&conn, "unknownword", "en"), "unknownword");
    }

    #[test]
    fn empty_input_returns_empty() {
        let conn = test_connection();
        assert_eq!(lemmatize(&conn, "   ", "en"), "");
    }
}
