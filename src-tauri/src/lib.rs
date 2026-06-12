// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod legendary;
mod system;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!, Greetings", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("Error while running tauri app");
    system::run_checks();
}
