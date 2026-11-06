use std::thread;
use std::time::Duration;

use arboard::Clipboard;

/// Tenta capturar o texto selecionado em qualquer app: lê o clipboard,
/// simula Ctrl+C, lê de novo, e sempre restaura o clipboard original
/// antes de retornar. A simulação de Ctrl+C só existe no Windows (única
/// plataforma de distribuição do app); em outras plataformas a função
/// sempre retorna `None`, o que abre a janela em modo de busca manual —
/// suficiente para testar o resto do fluxo fora do Windows.
pub fn capture_selected_text() -> Option<String> {
    let mut clipboard = Clipboard::new().ok()?;
    let original = clipboard.get_text().ok();

    send_copy_shortcut();
    thread::sleep(Duration::from_millis(80));

    let captured = clipboard.get_text().ok();

    match &original {
        Some(text) => {
            let _ = clipboard.set_text(text.clone());
        }
        None => {
            let _ = clipboard.clear();
        }
    }

    match captured {
        Some(new_text) if !new_text.trim().is_empty() && Some(&new_text) != original.as_ref() => {
            Some(new_text)
        }
        _ => None,
    }
}

#[cfg(target_os = "windows")]
fn send_copy_shortcut() {
    use windows::Win32::UI::Input::KeyboardAndMouse::{
        SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS,
        KEYEVENTF_KEYUP, VIRTUAL_KEY, VK_C, VK_CONTROL,
    };

    fn key_input(vk: VIRTUAL_KEY, flags: KEYBD_EVENT_FLAGS) -> INPUT {
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: vk,
                    wScan: 0,
                    dwFlags: flags,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        }
    }

    let inputs = [
        key_input(VK_CONTROL, KEYBD_EVENT_FLAGS(0)),
        key_input(VK_C, KEYBD_EVENT_FLAGS(0)),
        key_input(VK_C, KEYEVENTF_KEYUP),
        key_input(VK_CONTROL, KEYEVENTF_KEYUP),
    ];

    unsafe {
        SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
    }
}

#[cfg(not(target_os = "windows"))]
fn send_copy_shortcut() {}
