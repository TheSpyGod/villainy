// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod legendary;
mod system;

#[tauri::command]
fn startup() {
    system::run_checks();
    legendary::controller::log_in();
}

#[tauri::command]
fn start_game(title: &str) {
    legendary::controller::launch_game(title);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![startup, start_game])
        .run(tauri::generate_context!())
        .expect("Error while running tauri app");
}
