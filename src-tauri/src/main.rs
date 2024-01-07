// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    println!("name: {}", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn greet2() {
    println!("Hello from greet2");
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, greet2])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
