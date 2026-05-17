use std::path::Path;
use std::process::Command;
use chrono::Utc;
use serde::Deserialize;
use crate::models::{SessionStatus, Store};
use crate::utils::paths::Paths;

// Legendary outputs this shape from `legendary status --json`
#[derive(Deserialize)]
struct LegendaryStatus {
    account: Option<String>,
}

// GOGdl outputs this shape from `gogdl auth --print-token` / status equivalent
#[derive(Deserialize)]
struct GogdlStatus {
    username: Option<String>,
}

fn now() -> String {
    Utc::now().to_rfc3339()
}

fn unauthenticated(store: Store) -> SessionStatus {
    SessionStatus { store, authenticated: false, username: None, last_validated: now() }
}

fn check_legendary(binary: &Path) -> SessionStatus {
    let output = Command::new(binary)
        .arg("status")
        .arg("--json")
        .output();

    match output {
        Ok(o) if o.status.success() => {
            let parsed: Result<LegendaryStatus, _> = serde_json::from_slice(&o.stdout);
            SessionStatus {
                store: Store::Epic,
                authenticated: true,
                username: parsed.ok().and_then(|s| s.account),
                last_validated: now(),
            }
        }
        _ => unauthenticated(Store::Epic),
    }
}

fn check_gogdl(binary: &Path) -> SessionStatus {
    // GOGdl: `gogdl auth --print-token` exits 0 if authenticated
    let output = Command::new(binary)
        .arg("auth")
        .arg("--print-token")
        .output();

    match output {
        Ok(o) if o.status.success() => {
            let parsed: Result<GogdlStatus, _> = serde_json::from_slice(&o.stdout);
            SessionStatus {
                store: Store::Gog,
                authenticated: true,
                username: parsed.ok().and_then(|s| s.username),
                last_validated: now(),
            }
        }
        _ => unauthenticated(Store::Gog),
    }
}

fn check_nile(binary: &Path) -> SessionStatus {
    // Nile: `nile status` exits 0 if authenticated
    let output = Command::new(binary)
        .arg("status")
        .output();

    let authenticated = matches!(output, Ok(o) if o.status.success());
    SessionStatus {
        store: Store::Amazon,
        authenticated,
        username: None,
        last_validated: now(),
    }
}

// Lutris is local — no auth concept, always considered available if installed
fn check_lutris(binary: &Path) -> SessionStatus {
    let reachable = Command::new(binary).arg("--version").output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    SessionStatus {
        store: Store::Lutris,
        authenticated: reachable,
        username: None,
        last_validated: now(),
    }
}

pub fn validate_all(paths: &Paths) -> Vec<SessionStatus> {
    let mut statuses = Vec::new();

    if let Some(ref p) = paths.legendary { statuses.push(check_legendary(p)); }
    if let Some(ref p) = paths.gogdl     { statuses.push(check_gogdl(p));     }
    if let Some(ref p) = paths.nile      { statuses.push(check_nile(p));      }
    if let Some(ref p) = paths.lutris    { statuses.push(check_lutris(p));    }

    statuses
}
