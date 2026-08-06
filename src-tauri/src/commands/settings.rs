use crate::storage::settings::{self, AppSettings};
use crate::AppResult;
use tauri::{Emitter, Manager};
use tauri_plugin_autostart::ManagerExt;

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

    if settings.launch_at_startup {
        app.autolaunch()
            .enable()
            .map_err(|e| crate::error::AppError::StorageError(format!("Failed to enable Windows startup: {e}")))?;
    } else {
        app.autolaunch()
            .disable()
            .map_err(|e| crate::error::AppError::StorageError(format!("Failed to disable Windows startup: {e}")))?;
    }

    let _ = app.emit("settings-updated", &settings);

    Ok(())
}
