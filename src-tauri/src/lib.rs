mod commands;
mod db;
mod state;

use std::fs;
use std::sync::Mutex;

use tauri::Manager;

use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir()?;
            fs::create_dir_all(&app_data_dir)?;

            let db_path = app_data_dir.join("lexidesk.db");
            let conn = db::open_connection(&db_path)
                .map_err(|err| -> Box<dyn std::error::Error> { err.to_string().into() })?;

            app.manage(AppState {
                db: Mutex::new(conn),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![commands::db_health])
        .run(tauri::generate_context!())
        .expect("erro ao iniciar o LexiDesk");
}
