// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod document;
mod helpers;

use document::{generate_cpf, validate_cpf};
use tauri::{Manager, SystemTray, SystemTrayEvent};
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};
use tauri_plugin_positioner::{WindowExt, Position};
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial, NSVisualEffectState};


fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .invoke_handler(tauri::generate_handler![generate_cpf, validate_cpf])
        .system_tray(mount_base_tauri())
        .setup(|app| {
            #[cfg(target_os = "macos")]
            apply_vibrancy(
                app.get_window("main").unwrap(),
                NSVisualEffectMaterial::Menu,
                Some(NSVisualEffectState::Active),
                Some(8.0)
            ).expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            Ok(())
        }).on_system_tray_event(|app, event| {
            tauri_plugin_positioner::on_tray_event(app, &event);

            match event {
                SystemTrayEvent::LeftClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    let win = app.get_window("main").unwrap();
                    let _ = win.move_window(Position::TrayCenter);

                    win.show().unwrap();
                    win.set_focus().unwrap();
                }
                _ => {}
            }
        }).run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn mount_base_tauri() -> SystemTray {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);

    let system_tray = SystemTray::new()
        .with_menu(tray_menu);

    return system_tray
}
