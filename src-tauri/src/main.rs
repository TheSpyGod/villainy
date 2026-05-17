// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod cli;
mod session;
mod launch;
mod download;
mod models;
mod utils;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::library::get_library,
            commands::session::get_session_status,
            commands::session::validate_sessions,
            commands::session::authenticate,
            commands::session::logout,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Villainy");
}
