use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use semver::Version;
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::error::{AppError, AppResult};
use crate::storage::settings::{self, UpdateAssetMetadata};

const GITHUB_REPO: &str = "NitroQ/openrouter-usage-widget";

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseAsset {
    pub name: String,
    pub download_url: String,
    pub size: u64,
    pub signature_url: String,
    pub sha256: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInfo {
    pub current_version: String,
    pub latest_version: String,
    pub update_available: bool,
    pub release_url: String,
    pub assets: Vec<ReleaseAsset>,
    pub release_tag: String,
    pub compatible_asset_available: bool,
    pub asset: Option<ReleaseAsset>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStatus {
    pub current_version: String,
    pub last_check_at: Option<String>,
    pub update_available: bool,
    pub latest_version: Option<String>,
    pub compatible_asset_available: bool,
    pub asset: Option<ReleaseAsset>,
}

#[derive(Debug, Deserialize)]
struct GithubRelease {
    tag_name: String,
    html_url: String,
    assets: Vec<GithubAsset>,
}

#[derive(Debug, Deserialize)]
struct GithubAsset {
    name: String,
    browser_download_url: String,
    size: u64,
}

fn current_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

fn parse_semver(version: &str) -> Option<Version> {
    Version::parse(version.trim().strip_prefix('v').unwrap_or(version.trim())).ok()
}

fn is_newer(latest: &str, current: &str) -> bool {
    match (parse_semver(latest), parse_semver(current)) {
        (Some(latest), Some(current)) => latest > current,
        _ => false,
    }
}

fn platform_asset_tokens() -> Option<(&'static str, &'static str, &'static str)> {
    if cfg!(all(target_os = "windows", target_arch = "x86_64")) {
        Some((".exe", "windows", "x64"))
    } else if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
        Some((".deb", "linux", "amd64"))
    } else {
        None
    }
}

fn is_safe_asset_name(name: &str) -> bool {
    let path = Path::new(name);
    !name.is_empty()
        && path.file_name().and_then(|value| value.to_str()) == Some(name)
        && !name.contains("..")
}

fn select_asset(release: &GithubRelease) -> AppResult<Option<ReleaseAsset>> {
    let Some((extension, os, arch)) = platform_asset_tokens() else {
        return Ok(None);
    };
    let candidates: Vec<&GithubAsset> = release.assets.iter().filter(|asset| {
        asset.name.starts_with("openrouter-widget_")
            && asset.name.contains(os)
            && asset.name.contains(arch)
            && asset.name.ends_with(extension)
            && is_safe_asset_name(&asset.name)
    }).collect();
    if candidates.len() > 1 {
        return Err(AppError::InvalidInput("Multiple compatible update packages found".into()));
    }
    let Some(asset) = candidates.first() else {
        return Ok(None);
    };
    let signature_name = format!("{}.sig", asset.name);
    let signature = release.assets.iter().find(|candidate| candidate.name == signature_name)
        .ok_or_else(|| AppError::NotFound("No detached signature found for update package".into()))?;
    Ok(Some(ReleaseAsset {
        name: asset.name.clone(),
        download_url: asset.browser_download_url.clone(),
        size: asset.size,
        signature_url: signature.browser_download_url.clone(),
        sha256: None,
    }))
}

fn verify_signature(bytes: &[u8], signature_bytes: &[u8]) -> AppResult<()> {
    let encoded_key = option_env!("OPENROUTER_WIDGET_UPDATE_PUBLIC_KEY_HEX")
        .ok_or_else(|| AppError::InvalidInput("Update verification key is not configured".into()))?;
    let key_bytes = hex::decode(encoded_key)
        .map_err(|_| AppError::InvalidInput("Invalid update verification key".into()))?;
    let key_array: [u8; 32] = key_bytes.try_into()
        .map_err(|_| AppError::InvalidInput("Update verification key must be 32 bytes".into()))?;
    let key = VerifyingKey::from_bytes(&key_array)
        .map_err(|_| AppError::InvalidInput("Invalid update verification key".into()))?;
    let signature_text = std::str::from_utf8(signature_bytes).unwrap_or("").trim();
    let signature_data = hex::decode(signature_text)
        .map_err(|_| AppError::InvalidInput("Invalid update signature".into()))?;
    let signature = Signature::from_slice(&signature_data)
        .map_err(|_| AppError::InvalidInput("Invalid update signature".into()))?;
    key.verify(bytes, &signature)
        .map_err(|_| AppError::InvalidInput("Update signature verification failed".into()))
}

#[tauri::command]
pub async fn check_for_updates() -> AppResult<UpdateInfo> {
    let url = format!("https://api.github.com/repos/{GITHUB_REPO}/releases/latest");

    let client = reqwest::Client::builder()
        .user_agent("openrouter-widget-updater")
        .build()
        .map_err(|e| AppError::NetworkError(format!("Failed to create HTTP client: {e}")))?;

    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| AppError::NetworkError(format!("Failed to check for updates: {e}")))?;

    if !resp.status().is_success() {
        return Err(AppError::NetworkError(format!(
            "GitHub API returned status {}",
            resp.status()
        )));
    }

    let release: GithubRelease = resp
        .json()
        .await
        .map_err(|e| AppError::NetworkError(format!("Failed to parse release data: {e}")))?;

    let release_tag = release.tag_name.trim().to_string();
    let latest = release_tag.trim_start_matches('v').to_string();
    if parse_semver(&latest).is_none() {
        return Err(AppError::InvalidInput("Latest release has an invalid version tag".into()));
    }
    let current = current_version();
    let update_available = is_newer(&latest, &current);
    let asset = if update_available { select_asset(&release)? } else { None };
    let assets = asset.clone().into_iter().collect::<Vec<_>>();
    settings::save_update_metadata(
        chrono::Utc::now().to_rfc3339(),
        release_tag.clone(),
        latest.clone(),
        asset.clone().map(|value| UpdateAssetMetadata {
            name: value.name,
            download_url: value.download_url,
            signature_url: value.signature_url,
            size: value.size,
            sha256: value.sha256,
        }),
    )?;

    Ok(UpdateInfo {
        current_version: current,
        latest_version: latest,
        update_available,
        release_url: release.html_url,
        assets,
        release_tag,
        compatible_asset_available: asset.is_some(),
        asset,
    })
}

#[tauri::command]
pub async fn get_update_status() -> AppResult<UpdateStatus> {
    let s = settings::get_settings()?;
    let current = current_version();
    Ok(UpdateStatus {
        current_version: current.clone(),
        last_check_at: s.last_update_check_at,
        update_available: s.last_notified_version.as_deref().map(|version| is_newer(version, &current)).unwrap_or(false),
        latest_version: s.last_notified_version,
        compatible_asset_available: s.last_update_asset.is_some(),
        asset: s.last_update_asset.map(|asset| ReleaseAsset {
            name: asset.name,
            download_url: asset.download_url,
            size: asset.size,
            signature_url: asset.signature_url,
            sha256: asset.sha256,
        }),
    })
}

#[tauri::command]
pub async fn download_and_install_update(
    app: tauri::AppHandle,
    release_tag: String,
    asset_name: String,
) -> AppResult<()> {
    if !is_safe_asset_name(&asset_name) {
        return Err(AppError::InvalidInput("Invalid update asset name".into()));
    }
    let s = settings::get_settings()?;
    let cached = s.last_update_asset.ok_or_else(|| AppError::NotFound("No compatible cached update package".into()))?;
    if s.last_update_release_tag.as_deref() != Some(release_tag.as_str()) || cached.name != asset_name {
        return Err(AppError::InvalidInput("Update package is no longer current; check for updates again".into()));
    }
    let client = reqwest::Client::builder()
        .user_agent("openrouter-widget-updater")
        .build()
        .map_err(|e| AppError::NetworkError(format!("Failed to create HTTP client: {e}")))?;

    let download_resp = client
        .get(&cached.download_url)
        .send()
        .await
        .map_err(|e| AppError::NetworkError(format!("Failed to download update: {e}")))?;
    if !download_resp.status().is_success() {
        return Err(AppError::NetworkError(format!("Update download returned status {}", download_resp.status())));
    }
    if cached.size > 200 * 1024 * 1024 {
        return Err(AppError::InvalidInput("Update package exceeds the maximum size".into()));
    }
    let bytes = download_resp.bytes().await
        .map_err(|e| AppError::NetworkError(format!("Failed to read update data: {e}")))?;
    if bytes.len() as u64 > 200 * 1024 * 1024 {
        return Err(AppError::InvalidInput("Update package exceeds the maximum size".into()));
    }
    let file_path = tempfile::Builder::new().prefix("openrouter-widget-update-").suffix(if cfg!(target_os = "windows") { ".exe" } else { ".deb" }).tempfile()
        .map_err(|e| AppError::StorageError(format!("Failed to create update file: {e}")))?;
    let file_path_buf = file_path.path().to_path_buf();
    std::fs::write(&file_path_buf, &bytes)
        .map_err(|e| AppError::StorageError(format!("Failed to write update file: {e}")))?;
    let signature_bytes = client.get(&cached.signature_url).send().await
        .map_err(|e| AppError::NetworkError(format!("Failed to download update signature: {e}")))?;
    if !signature_bytes.status().is_success() {
        return Err(AppError::NetworkError(format!("Update signature returned status {}", signature_bytes.status())));
    }
    let signature_bytes = signature_bytes.bytes().await
        .map_err(|e| AppError::NetworkError(format!("Failed to read update signature: {e}")))?;
    verify_signature(&bytes, &signature_bytes)?;

    // Launch installer
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new(&file_path_buf)
            .spawn()
            .map_err(|e| AppError::StorageError(format!("Failed to launch installer: {e}")))?;
        file_path.keep().map_err(|e| AppError::StorageError(format!("Failed to preserve installer file: {e}")))?;
    }

    #[cfg(target_os = "linux")]
    {
        let status = std::process::Command::new("pkexec")
            .args(["dpkg", "-i", file_path_buf.to_str().unwrap_or("")])
            .status()
            .map_err(|e| AppError::StorageError(format!("Failed to launch installer: {e}")))?;
        if !status.success() {
            return Err(AppError::StorageError(format!("Installer exited with status {status}")));
        }
    }

    // Quit the application so the installer can replace files
    app.exit(0);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_semver() {
        assert_eq!(parse_semver("1.2.3"), Version::parse("1.2.3").ok());
        assert_eq!(parse_semver("v1.2.3"), Version::parse("1.2.3").ok());
    }

    #[test]
    fn parse_invalid_semver() {
        assert_eq!(parse_semver("abc"), None);
        assert_eq!(parse_semver("1.2"), None);
        assert_eq!(parse_semver(""), None);
    }

    #[test]
    fn newer_version_detected() {
        assert!(is_newer("0.2.0", "0.1.0"));
        assert!(is_newer("1.0.0", "0.9.9"));
        assert!(is_newer("0.1.1", "0.1.0"));
        assert!(is_newer("v0.2.0", "0.1.0"));
    }

    #[test]
    fn same_version_not_newer() {
        assert!(!is_newer("0.1.0", "0.1.0"));
    }

    #[test]
    fn older_version_not_newer() {
        assert!(!is_newer("0.1.0", "0.2.0"));
    }

    #[test]
    fn semver_prerelease_is_compared_correctly() {
        assert!(!is_newer("1.2.3-beta.1", "1.2.3"));
        assert!(is_newer("1.2.4-beta.1", "1.2.3"));
    }

    #[test]
    fn malformed_or_extra_version_components_are_rejected() {
        assert!(!is_newer("1.2.3.4", "1.2.3"));
        assert!(!is_newer("1.2", "1.1.0"));
    }

    #[test]
    fn unsafe_asset_names_are_rejected() {
        assert!(!is_safe_asset_name("../update.exe"));
        assert!(!is_safe_asset_name("nested/update.exe"));
        assert!(is_safe_asset_name("openrouter-widget_1.2.3_windows-x64.exe"));
    }
}
