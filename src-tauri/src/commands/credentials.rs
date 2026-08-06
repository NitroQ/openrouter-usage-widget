use tauri::{Emitter, Manager, State};

use crate::error::{AppError, AppResult};
use crate::openrouter::standard::ValidationResult;
use crate::storage::credentials as cred_store;
use crate::storage::database::Database;
use crate::storage::settings;

#[tauri::command]
pub async fn validate_and_save_credential(
    key: String,
    selected_mode: String,
    db: State<'_, Database>,
) -> AppResult<ValidationResult> {
    eprintln!("[validate_and_save_credential] mode={selected_mode}, key_len={}", key.len());
    let validation = match selected_mode.as_str() {
        "standard" => {
            crate::openrouter::standard::validate_standard_key(&key).await?
        }
        "management" => {
            crate::openrouter::management::validate_management_key(&key).await?
        }
        _ => {
            return Err(AppError::InvalidInput(format!(
                "Invalid mode: {selected_mode}. Must be 'standard' or 'management'."
            )));
        }
    };

    if !validation.valid {
        return Ok(validation);
    }

    let fingerprint = cred_store::compute_fingerprint(&key);

    cred_store::save_credential(&key)
        .map_err(|e| AppError::StorageError(e))?;

    db.create_credential_profile(
        &selected_mode,
        &fingerprint,
        validation.label.as_deref(),
    )?;

    Ok(validation)
}

#[tauri::command]
pub async fn replace_credential(
    key: String,
    selected_mode: String,
    db: State<'_, Database>,
) -> AppResult<ValidationResult> {
    let validation = match selected_mode.as_str() {
        "standard" => {
            crate::openrouter::standard::validate_standard_key(&key).await?
        }
        "management" => {
            crate::openrouter::management::validate_management_key(&key).await?
        }
        _ => {
            return Err(AppError::InvalidInput(format!(
                "Invalid mode: {selected_mode}. Must be 'standard' or 'management'."
            )));
        }
    };

    if !validation.valid {
        return Ok(validation);
    }

    let fingerprint = cred_store::compute_fingerprint(&key);

    // Delete old credential and save new one
    let _ = cred_store::delete_credential();
    cred_store::save_credential(&key)
        .map_err(|e| AppError::StorageError(e))?;

    db.create_credential_profile(
        &selected_mode,
        &fingerprint,
        validation.label.as_deref(),
    )?;

    Ok(validation)
}

#[tauri::command]
pub async fn forget_credential(db: State<'_, Database>) -> AppResult<()> {
    cred_store::delete_credential()
        .map_err(|e| AppError::StorageError(e))?;

    db.deactivate_all_profiles()?;

    Ok(())
}

#[tauri::command]
pub async fn reset_app_data(
    app: tauri::AppHandle,
    preserve_database: bool,
    db: State<'_, Database>,
) -> AppResult<()> {
    if cred_store::load_credential().is_some() {
        cred_store::delete_credential()
            .map_err(AppError::StorageError)?;
    }

    if preserve_database {
        db.deactivate_all_profiles()?;
    } else {
        db.clear_all_data()?;
    }

    let defaults = settings::AppSettings::default();
    settings::save_settings(&defaults)?;

    if let Some(window) = app.get_webview_window("main-widget") {
        let _ = window.set_always_on_top(defaults.always_on_top);
        let _ = window.set_skip_taskbar(!defaults.show_in_taskbar);
    }

    if defaults.launch_at_startup {
        let _ = tauri_plugin_autostart::ManagerExt::autolaunch(&app).enable();
    } else {
        let _ = tauri_plugin_autostart::ManagerExt::autolaunch(&app).disable();
    }

    let _ = app.emit("settings-updated", &defaults);

    app.restart();

    Ok(())
}
