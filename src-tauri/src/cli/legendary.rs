use std::path::Path;
use std::process::Command;
use serde::Deserialize;
use crate::models::{Game, Store};

#[derive(Deserialize)]
struct LegendaryGame {
    app_name: String,
    title: String,
    install_path: Option<String>,
}

pub fn list_installed(binary: &Path) -> Result<Vec<Game>, Box<dyn std::error::Error>> {
    let output = Command::new(binary)
        .arg("list-installed")
        .arg("--json")
        .output()?;

    let raw: Vec<LegendaryGame> = serde_json::from_slice(&output.stdout)?;

    let games = raw.into_iter().map(|g| Game {
        id: g.app_name,
        title: g.title,
        store: Store::Epic,
        installed: true,
        install_path: g.install_path,
        cover_url: None,
        playtime_secs: 0,
        last_played: None,
        is_running: false,
    }).collect();

    Ok(games)
}
