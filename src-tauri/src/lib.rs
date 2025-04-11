// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::{AppHandle, Builder, Wry};

// Re-export modules
pub mod database;
pub mod error;
pub mod models;
pub mod handlers;
pub mod utils;

#[cfg(mobile)]
mod mobile;

#[cfg(mobile)]
pub use mobile::*;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub fn run() {
    Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
