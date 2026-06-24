use regex::Regex;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub display_name: String,
    pub app_name: String,
    pub version: String,
    pub is_dependency: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameListResponse {
    pub games: Vec<Game>,
    pub total: usize,
}

#[tauri::command]
pub fn launch_game<'a, 'b>(game_name: &str) -> &'a bool {
    let output = Command::new("../venv/bin/legendary")
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
pub fn list_games() -> Result<GameListResponse, String> {
    let output = Command::new("../venv/bin/legendary")
        .arg("list")
        .output()
        .map_err(|e| {
            format!(
                "Failed to run 'legendary' executable. Is it installed? Error: {}",
                e
            )
        })?;

    let stdout_str = String::from_utf8_lossy(&output.stdout);
    let stderr_str = String::from_utf8_lossy(&output.stderr);
    let combined_output = format!("{}\n{}", stdout_str, stderr_str);

    if !output.status.success() {
        return Err(format!(
            "Legendary exited with an error (Code {}).\nLog output:\n{}",
            output.status.code().unwrap_or(-1),
            combined_output.trim()
        ));
    }

    let game_regex =
        Regex::new(r"(?m)^\s*([\*\+])\s*(.*?)\s*\(App name:\s*(\S+)\s*\|\s*Version:\s*([^\)]+)\)")
            .map_err(|e| format!("Internal tool error (Regex failure): {}", e))?;

    let mut games = Vec::new();

    for caps in game_regex.captures_iter(&combined_output) {
        // Safe access to capture groups because the regex match guarantees their existence
        let marker = caps.get(1).map(|m| m.as_str()).unwrap_or("*");
        let display_name = caps
            .get(2)
            .map(|m| m.as_str().trim().to_string())
            .unwrap_or_default();
        let app_name = caps
            .get(3)
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();
        let version = caps
            .get(4)
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();

        games.push(Game {
            display_name,
            app_name,
            version,
            is_dependency: marker == "+",
        });
    }

    if games.is_empty() && combined_output.contains("Available games:") {
        return Err(
            "No games found in your legendary library or output format has changed.".to_string(),
        );
    }

    Ok(GameListResponse {
        total: games.len(),
        games,
    })
}

#[tauri::command]
pub fn log_in<'a>(code: String) -> &'a bool {
    let output = Command::new("./venv/bin/legendary")
        .arg("auth")
        .arg("--code")
        .arg(code)
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        return &true;
    }

    return &false;
}
