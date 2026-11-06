mod dictionary;
mod settings;
mod window;

pub use dictionary::*;
pub use settings::*;
pub use window::*;

use tauri::State;

use crate::state::AppState;

#[tauri::command]
pub fn db_health(state: State<AppState>) -> Result<String, String> {
    let conn = state.db.lock().map_err(|err| err.to_string())?;
    let config_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM settings", [], |row| row.get(0))
        .map_err(|err| err.to_string())?;
    Ok(format!("ok ({config_count} configs)"))
}
