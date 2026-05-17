use std::path::PathBuf;
use std::process::Command;

fn find_binary(name: &str) -> Option<PathBuf> {
    // 1. Check PATH
    if let Ok(output) = Command::new("which").arg(name).output() {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            return Some(PathBuf::from(path));
        }
    }

    // 2. Flatpak system install
    let flatpak_system = PathBuf::from(format!("/var/lib/flatpak/exports/bin/{}", name));
    if flatpak_system.exists() {
        return Some(flatpak_system);
    }

    // 3. Flatpak user install
    if let Some(home) = std::env::var_os("HOME") {
        let flatpak_user = PathBuf::from(&home)
            .join(".local/share/flatpak/exports/bin")
            .join(name);
        if flatpak_user.exists() {
            return Some(flatpak_user);
        }

        // 4. User-local bin (e.g. pip install --user)
        let user_local = PathBuf::from(&home).join(".local/bin").join(name);
        if user_local.exists() {
            return Some(user_local);
        }
    }

    None
}

pub struct Paths {
    pub legendary: Option<PathBuf>,
    pub gogdl: Option<PathBuf>,
    pub nile: Option<PathBuf>,
    pub lutris: Option<PathBuf>,
}

impl Paths {
    pub fn resolve() -> Self {
        Paths {
            legendary: find_binary("legendary"),
            gogdl: find_binary("gogdl"),
            nile: find_binary("nile"),
            lutris: find_binary("lutris"),
        }
    }
}
