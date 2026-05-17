use crate::cli::legendary;
use crate::models::Game;
use crate::utils::paths::Paths;

#[tauri::command]
pub fn get_library() -> Result<Vec<Game>, String> {
    let paths = Paths::resolve();
    let mut games = Vec::new();

    if let Some(ref bin) = paths.legendary {
        match legendary::list_installed(bin) {
            Ok(mut epic) => games.append(&mut epic),
            Err(e) => eprintln!("[library] legendary: {}", e),
        }
    }

    Ok(games)
}
