use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Store {
    Epic,
    Gog,
    Amazon,
    Lutris,
    Sideload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: String,
    pub title: String,
    pub store: Store,
    pub installed: bool,
    pub install_path: Option<String>,
    pub cover_url: Option<String>,
    pub playtime_secs: u64,
    pub last_played: Option<String>,
    pub is_running: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadProgress {
    pub game_id: String,
    pub percent: f32,
    pub speed_mbps: f32,
    pub eta_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStatus {
    pub store: Store,
    pub authenticated: bool,
    pub username: Option<String>,
    pub last_validated: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub default_install_path: String,
    pub default_proton_version: String,
    pub max_concurrent_downloads: u32,
    pub enable_gamemode: bool,
    pub enable_mangohud: bool,
}
