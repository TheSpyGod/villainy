use serde::Deserialize;
use tokio::sync::oneshot;
use std::sync::Mutex;
use tauri::{WebviewUrl, WebviewWindowBuilder, AppHandle, Manager, Listener};
use url::Url;

#[derive(Deserialize)]
struct CallbackParams {
    code: String,
    state: String,
}

#[tauri::command]
pub async fn get_auth_code(app: AppHandle) -> Result<(), String> {
    match perform_auth(app).await {
        Ok(code) => {
            crate::legendary::controller::log_in(code);
            Ok(())
        }
        Err(e) => {
            eprintln!("Auth failed: {}", e);
            Err(e)
        }
    }
}

async fn perform_auth(app: AppHandle) -> Result<String, String> {
    let client_id = "34a02cf8f4414e29b15921876da36f9a";
    let legendary_redirect = "https://www.epicgames.com/id/api/redirect";
    let base_url = "https://www.epicgames.com/id/login";
    
    let mut oauth_url = Url::parse(base_url).map_err(|e| e.to_string())?;
    oauth_url.query_pairs_mut()
        .append_pair("redirectUrl", &format!("{}?clientId={}&responseType=code", legendary_redirect, client_id));

    let final_url = oauth_url.to_string();
    let login_url = WebviewUrl::External(Url::parse(&final_url).unwrap());

    let app_clone = app.clone();

    let _login_window = WebviewWindowBuilder::new(&app, "epic-login", login_url)
        .title("Log in with Epic Games")
        .inner_size(500.0, 650.0)
        .resizable(false)
        .build()
        .map_err(|e| e.to_string())?;

    Ok("ASDASDDS".into())
}
