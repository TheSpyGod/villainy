use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use crate::models::SessionStatus;

fn session_file() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    PathBuf::from(home).join(".local/share/villainy/session.json")
}

pub fn read_sessions() -> Vec<SessionStatus> {
    let path = session_file();
    if !path.exists() {
        return Vec::new();
    }
    let data = fs::read_to_string(&path).unwrap_or_default();
    serde_json::from_str(&data).unwrap_or_default()
}

pub fn write_sessions(statuses: &[SessionStatus]) -> Result<(), Box<dyn std::error::Error>> {
    let path = session_file();

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let data = serde_json::to_string_pretty(statuses)?;
    fs::write(&path, &data)?;

    // Owner read/write only — tokens are sensitive even if we don't store them directly
    fs::set_permissions(&path, fs::Permissions::from_mode(0o600))?;

    Ok(())
}
