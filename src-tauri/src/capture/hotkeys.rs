use mouse_position::mouse_position::Mouse;
use tauri::{AppHandle, Emitter};
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut, ShortcutState};

use crate::capture::selection::capture_selected_text;
use crate::window_manager::{show_dictionary_window, DICTIONARY_WINDOW_LABEL};

pub fn dictionary_shortcut() -> Shortcut {
    Shortcut::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyD)
}

/// Chamado pelo plugin de atalho global a cada tecla pressionada/solta de
/// um atalho registrado. Por enquanto só existe o atalho do dicionário;
/// o de captura de região (OCR) entra na Fase 9.
pub fn handle_shortcut(app: &AppHandle, shortcut: &Shortcut, state: ShortcutState) {
    if state != ShortcutState::Pressed {
        return;
    }
    if *shortcut != dictionary_shortcut() {
        return;
    }

    let text = capture_selected_text();
    let position = match Mouse::get_mouse_position() {
        Mouse::Position { x, y } => Some((x as f64, y as f64)),
        Mouse::Error => None,
    };

    if let Err(err) = show_dictionary_window(app, position) {
        eprintln!("falha ao abrir a janela do dicionário: {err}");
        return;
    }

    let _ = app.emit_to(DICTIONARY_WINDOW_LABEL, "dictionary:open", text);
}
