use serde::Serialize;
use tauri::State;

use crate::dictionary::{self, DictionaryEntry};
use crate::state::AppState;

#[derive(Serialize)]
pub struct DictionaryLookupResult {
    pub surface_form: String,
    pub lemma: String,
    pub language_code: Option<String>,
    pub entry: Option<DictionaryEntry>,
    pub known: bool,
}

#[tauri::command]
pub fn dictionary_lookup(
    state: State<AppState>,
    text: String,
) -> Result<DictionaryLookupResult, String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Err("texto vazio".into());
    }

    let conn = state.db.lock().map_err(|err| err.to_string())?;

    let language_code = dictionary::detect_language(trimmed).unwrap_or("en");
    let lemma = dictionary::lemmatize(&conn, trimmed, language_code);
    let entry = dictionary::lookup_entry(&conn, &lemma, language_code);
    let known = dictionary::is_known(&conn, &lemma, language_code);

    Ok(DictionaryLookupResult {
        surface_form: trimmed.to_string(),
        lemma,
        language_code: Some(language_code.to_string()),
        entry,
        known,
    })
}

#[tauri::command]
pub fn dictionary_search(
    state: State<AppState>,
    query: String,
) -> Result<Vec<DictionaryEntry>, String> {
    let conn = state.db.lock().map_err(|err| err.to_string())?;
    Ok(dictionary::search_entries(&conn, &query, 20))
}
