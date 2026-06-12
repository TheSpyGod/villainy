// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod legendary;
mod system;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    system::run_checks();
}
