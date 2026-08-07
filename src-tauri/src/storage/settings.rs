use serde::{Deserialize, Serialize};

use crate::error::AppResult;
use std::sync::{Mutex, OnceLock};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAssetMetadata {
    pub name: String,
    pub download_url: String,
    pub signature_url: String,
    pub size: u64,
    pub sha256: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub configured: bool,
    #[serde(rename = "keyMode")]
    pub key_mode: String,
    #[serde(rename = "refreshIntervalSeconds")]
    pub refresh_interval_seconds: u64,
    #[serde(rename = "alwaysOnTop")]
    pub always_on_top: bool,
    #[serde(rename = "launchAtStartup")]
    pub launch_at_startup: bool,
    #[serde(rename = "closeToTray")]
    pub close_to_tray: bool,
    #[serde(rename = "startMinimized")]
    pub start_minimized: bool,
    pub theme: String,
    pub opacity: f64,
    #[serde(rename = "compactMode")]
    pub compact_mode: bool,
    #[serde(rename = "historyRetentionDays")]
    pub history_retention_days: i64,
    #[serde(rename = "historyDisplayTimezone", default = "default_history_timezone")]
    pub history_display_timezone: String,
    #[serde(rename = "showInTaskbar")]
    pub show_in_taskbar: bool,
    #[serde(rename = "refreshOnLaunch")]
    pub refresh_on_launch: bool,
    #[serde(rename = "restorePosition")]
    pub restore_position: bool,
    #[serde(rename = "diagnosticLogs")]
    pub diagnostic_logs: bool,
    #[serde(rename = "lastUpdateCheckAt", default)]
    pub last_update_check_at: Option<String>,
    #[serde(rename = "lastNotifiedVersion", default)]
    pub last_notified_version: Option<String>,
    #[serde(rename = "lastUpdateReleaseTag", default)]
    pub last_update_release_tag: Option<String>,
    #[serde(rename = "lastUpdateAsset", default)]
    pub last_update_asset: Option<UpdateAssetMetadata>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            configured: false,
            key_mode: "standard".into(),
            refresh_interval_seconds: 60,
            always_on_top: true,
            launch_at_startup: true,
            close_to_tray: true,
            start_minimized: true,
            theme: "system".into(),
            opacity: 0.9,
            compact_mode: true,
            history_retention_days: 365,
            history_display_timezone: "utc".into(),
            show_in_taskbar: false,
            refresh_on_launch: true,
            restore_position: true,
            diagnostic_logs: false,
            last_update_check_at: None,
            last_notified_version: None,
            last_update_release_tag: None,
            last_update_asset: None,
        }
    }
}

fn default_history_timezone() -> String {
    "utc".into()
}

/// Get the stored settings, falling back to defaults for missing fields.
pub fn get_settings() -> AppResult<AppSettings> {
    let _guard = settings_lock().lock().map_err(|_| crate::error::AppError::StorageError("Settings lock poisoned".into()))?;
    get_settings_unlocked()
}

/// Save settings to the JSON file.
pub fn save_settings(settings: &AppSettings) -> AppResult<()> {
    let _guard = settings_lock().lock().map_err(|_| crate::error::AppError::StorageError("Settings lock poisoned".into()))?;
    save_settings_unlocked(settings)
}

pub fn save_update_metadata(
    checked_at: String,
    release_tag: String,
    latest_version: String,
    asset: Option<UpdateAssetMetadata>,
) -> AppResult<()> {
    let _guard = settings_lock().lock().map_err(|_| crate::error::AppError::StorageError("Settings lock poisoned".into()))?;
    let mut settings = get_settings_unlocked()?;
    settings.last_update_check_at = Some(checked_at);
    settings.last_update_release_tag = Some(release_tag);
    settings.last_update_asset = asset;
    settings.last_notified_version = Some(latest_version);
    save_settings_unlocked(&settings)
}

fn save_settings_unlocked(settings: &AppSettings) -> AppResult<()> {
    let store_path = get_settings_path()?;
    let content = serde_json::to_string_pretty(settings)
        .map_err(|e| crate::error::AppError::StorageError(format!("Failed to serialize settings: {e}")))?;

    let temp_path = store_path.with_extension("json.tmp");
    std::fs::write(&temp_path, content)
        .map_err(|e| crate::error::AppError::StorageError(format!("Failed to write settings file: {e}")))?;
    #[cfg(windows)]
    if store_path.exists() {
        std::fs::remove_file(&store_path).map_err(|e| crate::error::AppError::StorageError(format!("Failed to replace settings file: {e}")))?;
    }
    std::fs::rename(&temp_path, &store_path)
        .map_err(|e| crate::error::AppError::StorageError(format!("Failed to commit settings file: {e}")))?;

    Ok(())
}

fn get_settings_unlocked() -> AppResult<AppSettings> {
    let store_path = get_settings_path()?;
    if !store_path.exists() {
        return Ok(AppSettings::default());
    }
    let content = std::fs::read_to_string(&store_path)
        .map_err(|e| crate::error::AppError::StorageError(format!("Failed to read settings file: {e}")))?;
    let mut settings: AppSettings = serde_json::from_str(&content)
        .unwrap_or_else(|_| AppSettings::default());
    settings.history_display_timezone = "utc".into();
    Ok(settings)
}

fn settings_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

fn get_settings_path() -> AppResult<std::path::PathBuf> {
    let mut path = dirs::config_dir()
        .ok_or_else(|| crate::error::AppError::StorageError("Failed to get config dir".into()))?;
    path.push("openrouter-widget");
    std::fs::create_dir_all(&path)
        .map_err(|e| crate::error::AppError::StorageError(format!("Failed to create config dir: {e}")))?;
    path.push("settings.json");
    Ok(path)
}
