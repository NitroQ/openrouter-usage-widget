use crate::storage::settings::{self, AppSettings};
use crate::AppResult;
use tauri::{Emitter, Manager};

#[tauri::command]
pub async fn get_settings() -> AppResult<AppSettings> {
    settings::get_settings()
}

#[tauri::command]
pub async fn save_settings(app: tauri::AppHandle, settings: AppSettings) -> AppResult<()> {
    settings::save_settings(&settings)?;

    // Apply window-affecting settings immediately
    if let Some(window) = app.get_webview_window("main-widget") {
        let _ = window.set_always_on_top(settings.always_on_top);
        let _ = window.set_skip_taskbar(!settings.show_in_taskbar);
    }

    let _ = app.emit("settings-updated", &settings);

    Ok(())
}
