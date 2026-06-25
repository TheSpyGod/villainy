use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio, Child};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use crate::db::Database;

static legendary: &str = "legendary";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Game {
    pub name: String,
    pub platform: String,
    pub id: String,
    pub launch_id: String,
    pub is_installed: bool,
}

fn get_home() -> String {
    std::env::var("HOME").unwrap_or_else(|_| ".".to_string())
}

fn is_game_installed_on_disk(_platform: &str, name: &str) -> bool {
    let base_path = format!("{}/Games", get_home());
    let path = Path::new(&base_path);
    if !path.exists() || !path.is_dir() { return false; }

    let target = name.to_lowercase().replace([' ', '_'], "");
    std::fs::read_dir(path).map(|entries| {
        entries.flatten().any(|e| {
            e.file_name().to_string_lossy().to_lowercase().replace([' ', '_'], "").contains(&target)
        })
    }).unwrap_or(false)
}

pub fn sanitize_folder_name(name: &str) -> String {
    name.replace('*', "").replace(' ', "_").trim().to_string()
}

pub async fn get_gog_games() -> Result<Vec<Game>, String> {
    let output = Command::new("./gogdl/gogdl")
                    .arg("list")
                    .env_remove("PYTHONHOME")
                    .env_remove("PYTHONPATH")
                    .env_remove("PYTHONEXECUTABLE")
                    .output()
                    .map_err(|e| e.to_string())?;
    if !output.status.success() { return Err(format!("GOG error: {}", String::from_utf8_lossy(&output.stderr))); }

    Ok(String::from_utf8_lossy(&output.stdout).lines().filter(|l| !l.is_empty()).map(|line| {
        let clean = line.replace('*', "").trim().to_string();
        Game {
            name: clean.clone(),
            platform: "gog".to_string(),
            id: format!("gog_{}", clean.to_lowercase().replace(' ', "_")),
            launch_id: clean,
            is_installed: line.starts_with('*'),
        }
    }).collect())
}

pub fn uninstall_game_logic(game: &Game) -> Result<String, String> {
    match game.platform.as_str() {
        "epic" => {
            let status = std::process::Command::new(legendary)
                .args(["uninstall", &game.name, "--yes"])
                .status().map_err(|e| e.to_string())?;
            if status.success() { Ok("uninstalled".to_string()) } else { Err("Epic uninstall failed".to_string()) }
        }
        "gog" => {
            let path = get_game_install_dir(&game.name, &game.launch_id, "gog")?;
            if path.exists() {
                std::fs::remove_dir_all(path).map_err(|e| e.to_string())?;
                Ok("uninstalled".to_string())
            } else { Err("Path not found".to_string()) }
        }
        _ => Err("Unknown platform".to_string()),
    }
}

async fn get_epic_games() -> Result<Vec<Game>, String> {
    let output = Command::new(legendary)
        .arg("list")
        .env_clear()
        .env("PATH", "/usr/bin:/bin:/usr/local/bin:/home/shreadr/.local/bin")
        .env("HOME", std::env::var("HOME").unwrap_or_else(|_| "/home/shreadr".to_string()))
        .env("LANG", "en_US.UTF-8")
        .env("HOME", get_home())
        .output()
        .map_err(|e| format!("legendary error: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // DEBUG: Zapisz wyjście do pliku, aby sprawdzić co widzi aplikacja
    let _ = std::fs::write(format!("{}/debug_legendary.txt", get_home()), stdout.as_bytes());

    if !output.status.success() {
        return Err(format!("Legendary failed: {}", String::from_utf8_lossy(&output.stderr)));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut installed_names = std::collections::HashSet::new();
    
    if let Some(installed_section) = stdout.split("Installed games:").nth(1) {
        for line in installed_section.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with('*') {
                if let Some(name) = trimmed.trim_start_matches('*').split(" (").next() {
                    installed_names.insert(name.trim().to_string());
                }
            }
        }
    }

    let mut games = Vec::new();
    let platform = "epic";

    if let Some(available_section) = stdout.split("Available games:").nth(1) {
        for line in available_section.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('+') || !trimmed.contains("App name:") {
                continue;
            }
            if trimmed.starts_with("Installed games:") { break; }

            let clean_line = trimmed.trim_start_matches('*').trim();

            if let Some(title_part) = clean_line.split(" (").next() {
                let game_name = title_part.trim().replace("*", "");

                if let Some(app_idx) = clean_line.find("App name: ") {
                    let app_id = clean_line[app_idx + 10..]
                        .split('|')
                        .next()
                        .unwrap_or("")
                        .trim()
                        .to_string();

                    if !game_name.is_empty() && !app_id.is_empty() {
                        games.push(Game {
                            id: format!("{}_{}", platform, game_name.to_lowercase().replace(" ", "_")),
                            name: game_name.clone(),
                            platform: platform.to_string(),
                            launch_id: app_id,
                            is_installed: is_game_installed_on_disk(platform, &game_name),
                        });
                    }
                }
            }
        }
    }
    Ok(games)
}

pub async fn get_all_games() -> Result<Vec<Game>, String> {
    let mut epic = get_epic_games().await.map_err(|e| {
        eprintln!("BŁĄD EPIC: {}", e);
        e
    })?;
    let mut gog = get_gog_games().await.unwrap_or_default();
    
    for game in epic.iter_mut().chain(gog.iter_mut()) {
        game.is_installed = is_game_installed_on_disk(&game.platform, &game.name);
    }
    
    epic.append(&mut gog);
    Ok(epic)
}

pub fn get_game_install_dir(name: &str, launch_id: &str, platform: &str) -> Result<PathBuf, String> {
    let home = get_home();
    match platform {
        "epic" => {
            let path = PathBuf::from(&home).join(".config/legendary/installed.json");
            if let Ok(content) = std::fs::read_to_string(path) {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(p) = json.get(launch_id).and_then(|v| v["install_path"].as_str()) {
                        return Ok(PathBuf::from(p));
                    }
                }
            }
            Ok(PathBuf::from(&home).join("legendary").join(launch_id))
        }
        "gog" => Ok(std::env::current_dir().unwrap_or_default().join("gogdl").join(name.replace(' ', "_"))),
        _ => Err("Unknown platform".to_string()),
    }
}

pub fn launch_game(game: Game, db_lock: Arc<Mutex<Database>>) -> Result<Child, String> {
    let home = get_home();
    let db = db_lock.lock().map_err(|e| e.to_string())?;
    let proton = db.get_setting("proton_path").unwrap_or_default();
    drop(db);

    let runner = if !proton.is_empty() && Path::new(&proton).exists() { proton } 
        else { ["/.local/share/Steam/steamapps/common/Proton - Experimental/proton", "/.var/app/com.valvesoftware.Steam/.steam/steam/steamapps/common/Proton - Experimental/proton"]
            .iter().map(|p| format!("{}{}", home, p)).find(|p| Path::new(p).exists()).unwrap_or_else(|| "/usr/bin/wine".to_string()) };

    let base = get_game_install_dir(&game.name, &game.launch_id, &game.platform)?;
    let exe = base.read_dir().ok().and_then(|d| d.flatten().find(|e| {
        let p = e.path();
        p.extension().map_or(false, |ex| ex == "exe") && !p.to_string_lossy().contains("benchmark")
    })).map(|e| e.path()).ok_or_else(|| format!("No exe in {:?}", base))?;

    let mut cmd = Command::new(&runner);
    cmd.current_dir(&base);

    if runner.ends_with("wine") {
        cmd.arg(&exe).env("WINEPREFIX", format!("{}/.wine", home));
    } else {
        cmd.args(["run", &exe.to_string_lossy()])
           .env("STEAM_COMPAT_DATA_PATH", format!("{}/.wine", home))
           .env("STEAM_COMPAT_CLIENT_INSTALL_PATH", format!("{}/.steam/steam", home))
           .env("PROTON_USE_WINED3D", "0");
    }

    for var in ["DISPLAY", "WAYLAND_DISPLAY", "XAUTHORITY"] {
        if let Ok(val) = std::env::var(var) { cmd.env(var, val); }
    }
    
    cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit()).spawn().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn install_game(game: Game) -> Result<String, String> {
    match game.platform.as_str() {
        "epic" => {
            let output = Command::new(legendary).args(["install", &game.name, "--yes", "--skip-sdl"]).output().map_err(|e| e.to_string())?;
            let out = format!("{}{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
            if out.contains("already up to date") || out.contains("Download size is 0") { Ok("already_installed".to_string()) }
            else if output.status.success() { Ok("installed".to_string()) } else { Err(out) }
        }
        "gog" => {
            Command::new("./gogdl/gogdl").args(["download", &game.name, "--yes"]).output().map_err(|e| e.to_string())?;
            if Path::new(&format!("{}/Games/{}", get_home(), sanitize_folder_name(&game.name))).exists() { Ok("installed".to_string()) }
            else { Err("install_failed".to_string()) }
        }
        _ => Err("Unknown platform".to_string()),
    }
}
