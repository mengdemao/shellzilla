// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

mod menu;

fn main() {
    let context = tauri::generate_context!();
    let _ = tauri::Builder::default()
        .menu(menu::init(&context))
        .on_menu_event(menu::handler)
        .run(context)
        .expect("error while running OhMyBox application");

    let _ = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running shellzilla application");

    // 设置webview打印信息
    let _ = tauri::Builder::default()
        .setup(|_app| {
          #[cfg(debug_assertions)] // only include this code on debug builds
          {
            let window = _app.get_window("main").unwrap();
            window.open_devtools();
            window.close_devtools();
          }
          Ok(())
        });
}
