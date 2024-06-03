// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod document;

use document::CPF;
use tauri::{Manager, SystemTray, SystemTrayEvent, Window};
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};
use tauri_plugin_positioner::{WindowExt, Position};
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial, NSVisualEffectState};


#[tauri::command]
fn generate_cpf() -> String {
    println!("{}", CPF::generate_document());
    return CPF::generate_document();
}

fn main() {
    let tray = SystemTray::new();

    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);

    let tray_menu = SystemTrayMenu::new();
    let system_tray = SystemTray::new()
        .with_menu(tray_menu);

    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .system_tray(system_tray)
        .setup(|app| {
            let handle = app.handle();
            let win: Window = app.get_window("main").unwrap();

            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            #[cfg(target_os = "macos")]
            apply_vibrancy(
                &win,
                NSVisualEffectMaterial::Menu,
                Some(NSVisualEffectState::Active),
                Some(8.0)
            ).expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

            Ok(())
        }).on_system_tray_event(|app, event| {
            tauri_plugin_positioner::on_tray_event(app, &event);

            match event {
                SystemTrayEvent::LeftClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    let mut win = app.get_window("main").unwrap();
                    let _ = win.move_window(Position::TrayCenter);

                    win.show().unwrap();
                    win.set_focus().unwrap();
                }
                _ => {}
            }
        }).run(tauri::generate_context!())
        .expect("error while running tauri application");
}
