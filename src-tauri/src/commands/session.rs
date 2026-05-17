use std::process::Command;
use crate::models::{SessionStatus, Store};
use crate::session::{store, validator};
use crate::utils::paths::Paths;

#[tauri::command]
pub fn get_session_status() -> Vec<SessionStatus> {
    store::read_sessions()
}

#[tauri::command]
pub fn validate_sessions() -> Vec<SessionStatus> {
    let paths = Paths::resolve();
    let statuses = validator::validate_all(&paths);
    let _ = store::write_sessions(&statuses);
    statuses
}

#[tauri::command]
pub fn authenticate(store_name: Store) -> Result<SessionStatus, String> {
    let paths = Paths::resolve();

    let binary = match store_name {
        Store::Epic    => paths.legendary,
        Store::Gog     => paths.gogdl,
        Store::Amazon  => paths.nile,
        Store::Lutris  => return Err("Lutris does not require authentication".to_string()),
        Store::Sideload => return Err("Sideloaded games do not require authentication".to_string()),
    };

    let binary = binary.ok_or_else(|| format!("CLI tool for {:?} not found", store_name))?;

    // Each tool's auth command opens a browser and blocks until the user completes login
    let status = Command::new(&binary)
        .arg("auth")
        .status()
        .map_err(|e| e.to_string())?;

    if !status.success() {
        return Err(format!("Authentication failed for {:?}", store_name));
    }

    // Re-validate after successful auth to get the fresh session status
    let paths = Paths::resolve();
    let all = validator::validate_all(&paths);
    let _ = store::write_sessions(&all);

    all.into_iter()
        .find(|s| std::mem::discriminant(&s.store) == std::mem::discriminant(&store_name))
        .ok_or_else(|| "Could not read session status after auth".to_string())
}

#[tauri::command]
pub fn logout(store_name: Store) -> Result<(), String> {
    let paths = Paths::resolve();

    let binary = match store_name {
        Store::Epic    => paths.legendary,
        Store::Gog     => paths.gogdl,
        Store::Amazon  => paths.nile,
        _              => return Err("Store does not support logout".to_string()),
    };

    let binary = binary.ok_or_else(|| format!("CLI tool for {:?} not found", store_name))?;

    Command::new(&binary)
        .arg("auth")
        .arg("--delete")
        .status()
        .map_err(|e| e.to_string())?;

    // Update session file to reflect logout
    let paths = Paths::resolve();
    let all = validator::validate_all(&paths);
    let _ = store::write_sessions(&all);

    Ok(())
}
