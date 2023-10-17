// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let _ = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
    // 设置webview打印信息
    use tauri::Manager;   
    let _ = tauri::Builder::default()
        .setup(|app| {
          #[cfg(debug_assertions)] // only include this code on debug builds
          {
            let window = app.get_window("main").unwrap();
            window.open_devtools();
            window.close_devtools();
          }
          Ok(())
        });
}
