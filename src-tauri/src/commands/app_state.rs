use serde::Serialize;

use crate::error::AppResult;
use crate::storage::credentials;
use crate::storage::database::Database;
use tauri::State;

#[derive(Serialize)]
pub struct AppState {
    pub configured: bool,
    pub key_mode: Option<String>,
    pub key_label: Option<String>,
}

#[tauri::command]
pub async fn get_app_state(db: State<'_, Database>) -> AppResult<AppState> {
    let profile = db.get_active_credential_profile()?;

    match profile {
        Some((_id, mode, _fingerprint, label)) => Ok(AppState {
            configured: true,
            key_mode: Some(mode),
            key_label: label,
        }),
        None => {
            // Check if there's a stored key but no profile (shouldn't happen normally)
            let has_key = credentials::load_credential().is_some();
            Ok(AppState {
                configured: has_key,
                key_mode: None,
                key_label: None,
            })
        }
    }
}
