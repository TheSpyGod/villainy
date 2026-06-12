use std::process::Command;

#[tauri::command]
pub fn launch_game<'a, 'b>(game_name: &str) -> &'a bool {
    let output = Command::new("legendary")
        .arg("launch")
        .arg(game_name)
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        return &true;
    }

    return &false;
}

#[tauri::command]
pub fn log_in<'a>() -> &'a bool {
    let output = Command::new("legendary")
        .arg("auth")
        .arg("login")
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        return &true;
    }

    return &false;
}
