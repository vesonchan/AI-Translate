use crate::{app_state::AppState, commands::start_area_selection, database};
use std::str::FromStr;
use tauri::AppHandle;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};
use tauri::Manager;

pub fn register_shortcuts(app: &AppHandle) {
    let state = app.state::<AppState>();
    let db = state.db.lock().unwrap();

    let _ = app.global_shortcut().unregister_all();

    let hotkeys = if let Ok(Some(config)) = db.get_app_config() {
        config.hotkeys
    } else {
        database::HotkeyConfig {
            popup_window: "Ctrl+Shift+T".to_string(),
            slide_translation: "Ctrl+Shift+S".to_string(),
            screenshot_translation: "Ctrl+Shift+A".to_string(),
        }
    };

    if !hotkeys.popup_window.is_empty() {
        if let Ok(shortcut) = Shortcut::from_str(&hotkeys.popup_window) {
            let app_handle = app.clone();
            match app.global_shortcut().register(shortcut) {
                Ok(_) => {
                    let _ = app.global_shortcut().on_shortcut(
                        shortcut,
                        move |_app, _shortcut, event| {
                            if event.state == ShortcutState::Pressed {
                                if let Some(window) = app_handle.get_webview_window("main") {
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                }
                            }
                        },
                    );
                    println!("Registered popup window shortcut: {}", hotkeys.popup_window);
                }
                Err(e) => {
                    eprintln!(
                        "Failed to register popup window shortcut '{}': {}",
                        hotkeys.popup_window, e
                    );
                }
            }
        }
    }

    if !hotkeys.screenshot_translation.is_empty() {
        if let Ok(shortcut) = Shortcut::from_str(&hotkeys.screenshot_translation) {
            let app_handle = app.clone();
            match app.global_shortcut().register(shortcut) {
                Ok(_) => {
                    let _ = app.global_shortcut().on_shortcut(
                        shortcut,
                        move |_app, _shortcut, event| {
                            if event.state == ShortcutState::Pressed {
                                let _ = start_area_selection(app_handle.clone());
                            }
                        },
                    );
                    println!(
                        "Registered screenshot translation shortcut: {}",
                        hotkeys.screenshot_translation
                    );
                }
                Err(e) => {
                    eprintln!(
                        "Failed to register screenshot translation shortcut '{}': {}",
                        hotkeys.screenshot_translation, e
                    );
                }
            }
        }
    }

    if !hotkeys.slide_translation.is_empty() {
        if let Ok(shortcut) = Shortcut::from_str(&hotkeys.slide_translation) {
            match app.global_shortcut().register(shortcut) {
                Ok(_) => {
                    println!(
                        "Registered slide translation shortcut: {}",
                        hotkeys.slide_translation
                    );
                }
                Err(e) => {
                    eprintln!(
                        "Failed to register slide translation shortcut '{}': {}",
                        hotkeys.slide_translation, e
                    );
                }
            }
        }
    }
}
