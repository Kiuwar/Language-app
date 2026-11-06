use tauri::{AppHandle, LogicalPosition, Manager, WebviewUrl, WebviewWindowBuilder};

pub const DICTIONARY_WINDOW_LABEL: &str = "dictionary";
const DICTIONARY_WINDOW_WIDTH: f64 = 360.0;
const DICTIONARY_WINDOW_HEIGHT: f64 = 240.0;

/// Cria a janela do dicionário oculta, uma única vez, no setup do app.
/// Ela nunca é destruída — abrir/fechar depois é só show()/hide(), o que
/// é o que viabiliza o requisito de abrir em menos de 300ms.
pub fn create_dictionary_window(app: &AppHandle) -> tauri::Result<()> {
    WebviewWindowBuilder::new(
        app,
        DICTIONARY_WINDOW_LABEL,
        WebviewUrl::App("dictionary-window.html".into()),
    )
    .title("LexiDesk - Dicionário")
    .inner_size(DICTIONARY_WINDOW_WIDTH, DICTIONARY_WINDOW_HEIGHT)
    .decorations(false)
    .always_on_top(true)
    .skip_taskbar(true)
    .resizable(false)
    .visible(false)
    .build()?;
    Ok(())
}

pub fn show_dictionary_window(app: &AppHandle, cursor: Option<(f64, f64)>) -> tauri::Result<()> {
    let Some(window) = app.get_webview_window(DICTIONARY_WINDOW_LABEL) else {
        return Ok(());
    };

    if let Some((x, y)) = cursor {
        let (clamped_x, clamped_y) = clamp_to_monitor(app, x, y);
        window.set_position(LogicalPosition::new(clamped_x, clamped_y))?;
    }

    window.show()?;
    window.set_focus()?;
    Ok(())
}

pub fn hide_dictionary_window(app: &AppHandle) -> tauri::Result<()> {
    if let Some(window) = app.get_webview_window(DICTIONARY_WINDOW_LABEL) {
        window.hide()?;
    }
    Ok(())
}

/// Garante que a janela (360x240) não abra fora da área visível do monitor
/// principal. Não cobre setups multi-monitor com escalas diferentes —
/// fica anotado como limitação conhecida (só validável no Windows real).
fn clamp_to_monitor(app: &AppHandle, x: f64, y: f64) -> (f64, f64) {
    match app.primary_monitor() {
        Ok(Some(monitor)) => {
            let scale = monitor.scale_factor();
            let size = monitor.size();
            let max_x = (size.width as f64 / scale) - DICTIONARY_WINDOW_WIDTH;
            let max_y = (size.height as f64 / scale) - DICTIONARY_WINDOW_HEIGHT;
            (x.clamp(0.0, max_x.max(0.0)), y.clamp(0.0, max_y.max(0.0)))
        }
        _ => (x, y),
    }
}
