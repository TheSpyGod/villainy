use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub id: String,
    pub title: String,
    pub installed: bool,
}

#[derive(Serialize, Deserialize)]
pub enum struct Store {
    Epic,
    Gog,
    Amazon,
    SideLoad,
}

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub default_install_path: String,
    pub default_proton_version: String,
    pub max_concurrent_downloads: u8,
}

#[derive(Serialize, Deserialize)]
pub struct SessionStatus {
    pub id: String,
    pub username: Option<String>,
    pub authenticated: bool,
    pub is_expired: bool,
}


