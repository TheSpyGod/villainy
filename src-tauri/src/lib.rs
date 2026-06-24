// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod legendary;
mod system;
mod redirect;

#[tauri::command]
fn start_game(title: &str) {
    legendary::controller::launch_game(title);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            start_game,
            legendary::controller::list_games,
            redirect::get_auth_code,
            legendary::controller::log_in

        ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri app");
}
