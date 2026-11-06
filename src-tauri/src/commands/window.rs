use tauri::{AppHandle, Emitter};

use crate::window_manager::{hide_dictionary_window, show_dictionary_window, DICTIONARY_WINDOW_LABEL};

/// Abre a janela do dicionário sem depender do atalho global — usado pela
/// paleta de comandos para testar o fluxo de dentro do app principal
/// (o atalho global de verdade, capturando seleção em qualquer janela do
/// SO, só é testável no Windows).
#[tauri::command]
pub fn open_dictionary_window(app: AppHandle, text: Option<String>) -> Result<(), String> {
    show_dictionary_window(&app, None).map_err(|err| err.to_string())?;
    let _ = app.emit_to(DICTIONARY_WINDOW_LABEL, "dictionary:open", text);
    Ok(())
}

#[tauri::command]
pub fn close_dictionary_window(app: AppHandle) -> Result<(), String> {
    hide_dictionary_window(&app).map_err(|err| err.to_string())
}
