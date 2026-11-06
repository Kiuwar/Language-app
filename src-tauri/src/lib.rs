mod capture;
mod commands;
mod db;
mod dictionary;
mod state;
mod window_manager;

use std::fs;
use std::sync::Mutex;

use tauri::path::BaseDirectory;
use tauri::Manager;

use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    capture::handle_shortcut(app, shortcut, event.state());
                })
                .build(),
        )
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir()?;
            fs::create_dir_all(&app_data_dir)?;

            let db_path = app_data_dir.join("lexidesk.db");
            let conn = db::open_connection(&db_path)
                .map_err(|err| -> Box<dyn std::error::Error> { err.to_string().into() })?;

            let dict_path = app
                .path()
                .resolve("dict/dictionary.db", BaseDirectory::Resource)?;
            conn.execute(
                "ATTACH DATABASE ?1 AS dict",
                [dict_path.to_string_lossy().to_string()],
            )?;

            app.manage(AppState {
                db: Mutex::new(conn),
            });

            window_manager::create_dictionary_window(app.handle())?;

            use tauri_plugin_global_shortcut::GlobalShortcutExt;
            app.global_shortcut()
                .register(capture::dictionary_shortcut())?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::db_health,
            commands::get_setting,
            commands::set_setting,
            commands::dictionary_lookup,
            commands::dictionary_search,
            commands::open_dictionary_window,
            commands::close_dictionary_window,
        ])
        .run(tauri::generate_context!())
        .expect("erro ao iniciar o LexiDesk");
}
