mod games;
mod db;
mod stats;
 
use rusqlite::params;
use games::Game;
use db::{Database, GameStats};
use stats::StatsTracker;
use std::sync::{Arc, Mutex};
use tauri::State;
 
pub struct AppState {
    db: Arc<Mutex<Database>>,
    stats_tracker: Arc<Mutex<StatsTracker>>,
}

#[tauri::command]
async fn rate_game(
    game_name: String,
    platform: String,
    rating: i32,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.update_game_rating(&game_name, &platform, rating).map_err(|e| e.to_string())
}
 
#[tauri::command]
async fn fetch_all_games() -> Result<Vec<Game>, String> {
    games::get_all_games().await
}
 
#[tauri::command]
async fn install_game_command(game: Game) -> Result<String, String> {
    games::install_game(game).await
}

#[tauri::command]
async fn get_all_stats(state: State<'_, AppState>) -> Result<Vec<GameStats>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.get_all_game_stats().map_err(|e| e.to_string())
}

#[tauri::command]
async fn uninstall_game_command(game: Game) -> Result<String, String> {
    games::uninstall_game_logic(&game)
}
 
#[tauri::command]
async fn record_playtime(
    game_name: String,
    platform: String,
    duration_minutes: i32,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.record_play_session(&game_name, &platform, duration_minutes).map_err(|e| e.to_string())
}
 
#[tauri::command]
async fn get_active_sessions() -> Result<Vec<String>, String> {
    let mut active = Vec::new();

    if std::process::Command::new("legendary")
        .arg("auth")
        .output()
        .ok()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        active.push("epic".to_string());
    }

    if std::process::Command::new("gogdl")
        .arg("login")
        .output()
        .ok()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        active.push("gog".to_string());
    }

    Ok(active)
}

#[tauri::command]
async fn logout_platform(platform: String) -> Result<(), String> {
    match platform.as_str() {
        "epic" => {
            std::process::Command::new("legendary")
                .arg("auth")
                .arg("--delete")
                .output()
                .map_err(|e| e.to_string())?;
            Ok(())
        }
        "gog" => {
            std::process::Command::new("gogdl")
                .arg("logout")
                .output()
                .map_err(|e| e.to_string())?;
            Ok(())
        }
        _ => Err("Unknown platform".to_string()),
    }
}

#[tauri::command]
async fn get_proton_path_command(state: State<'_, AppState>) -> Result<String, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.get_setting("proton_path").map_err(|e| e.to_string())
}

#[tauri::command]
async fn save_proton_path_command(path: String, state: State<'_, AppState>) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.save_setting("proton_path", &path).map_err(|e| e.to_string())
}
 
#[tauri::command]
async fn launch_game_tracked(
    game: Game,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let mut child_process = games::launch_game(game.clone(), state.db.clone())?; 
    
    let game_name = game.name.clone();
    let platform = game.platform.clone();
    let db = state.db.clone();

    tokio::spawn(async move {
        let start_inst = std::time::Instant::now();
        
        loop {
            match child_process.try_wait() {
                Ok(Some(_status)) => {
                    let total_minutes = (start_inst.elapsed().as_secs() / 60) as i32;
                    let runtime_addition = std::cmp::max(1, total_minutes);
                    
                    if let Ok(db_lock) = db.lock() {
                        let _ = db_lock.record_play_session(&game_name, &platform, runtime_addition);
                    }
                    break;
                }
                Ok(None) => {
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                }
                Err(_) => break,
            }
        }
    });

    Ok(format!("Launched {}", game.name))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let db = Database::new().expect("Failed to initialize database");
    let app_state = AppState {
        db: Arc::new(Mutex::new(db)),
        stats_tracker: Arc::new(Mutex::new(StatsTracker::new())),
    };
 
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            fetch_all_games,
            get_proton_path_command,
            save_proton_path_command,
            launch_game_tracked,
            install_game_command,
            rate_game,
            uninstall_game_command,
            get_all_stats,
            record_playtime,
            get_active_sessions,
            logout_platform,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
