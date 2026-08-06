use tauri::Manager;
use crate::error::AppResult;

#[tauri::command]
pub async fn show_widget(app: tauri::AppHandle) -> AppResult<()> {
    if let Some(window) = app.get_webview_window("main-widget") {
        window.show().map_err(|e| crate::error::AppError::StorageError(format!("Failed to show widget: {e}")))?;
        window.set_focus().map_err(|e| crate::error::AppError::StorageError(format!("Failed to focus widget: {e}")))?;
    }
    Ok(())
}

#[tauri::command]
pub async fn show_settings(app: tauri::AppHandle) -> AppResult<()> {
    if let Some(window) = app.get_webview_window("settings") {
        window.show().map_err(|e| crate::error::AppError::StorageError(format!("Failed to show settings: {e}")))?;
        window.set_focus().map_err(|e| crate::error::AppError::StorageError(format!("Failed to focus settings: {e}")))?;
    } else {
        let _ = tauri::WebviewWindowBuilder::new(
            &app,
            "settings",
            tauri::WebviewUrl::App("index.html#/settings".into()),
        )
        .title("Settings")
        .inner_size(760.0, 680.0)
        .min_inner_size(640.0, 520.0)
        .resizable(true)
        .decorations(true)
        .visible(true)
        .build()
        .map_err(|e| crate::error::AppError::StorageError(format!("Failed to create settings window: {e}")))?;
    }
    Ok(())
}

#[tauri::command]
pub async fn quit_application(app: tauri::AppHandle) {
    app.exit(0);
}
