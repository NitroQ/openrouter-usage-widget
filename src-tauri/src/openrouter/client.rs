use reqwest::Client;
use reqwest::StatusCode;

use crate::error::{AppError, AppResult};
use super::models::*;

const BASE_URL: &str = "https://openrouter.ai";
const TIMEOUT_SECS: u64 = 10;

fn build_client() -> AppResult<Client> {
    Client::builder()
        .timeout(std::time::Duration::from_secs(TIMEOUT_SECS))
        .build()
        .map_err(|e| AppError::NetworkError(format!("Failed to create HTTP client: {e}")))
}

fn auth_header(api_key: &str) -> String {
    format!("Bearer {api_key}")
}

async fn handle_response(resp: reqwest::Response) -> AppResult<String> {
    let status = resp.status();
    let text = resp
        .text()
        .await
        .map_err(|e| AppError::NetworkError(format!("Failed to read response body: {e}")))?;

    if status == StatusCode::UNAUTHORIZED {
        return Err(AppError::AuthError(
            "Invalid API key or key has been revoked".into(),
        ));
    }
    if status == StatusCode::FORBIDDEN {
        return Err(AppError::AuthError(
            "Access forbidden. Check your API key permissions.".into(),
        ));
    }
    if status.is_server_error() {
        return Err(AppError::NetworkError(format!(
            "OpenRouter server error ({status}): {text}"
        )));
    }
    if !status.is_success() {
        return Err(AppError::OpenRouterError(format!(
            "Request failed with status {status}: {text}"
        )));
    }

    Ok(text)
}

pub async fn get_key_info(api_key: &str) -> AppResult<CurrentKeyData> {
    let client = build_client()?;
    let resp = client
        .get(format!("{BASE_URL}/api/v1/auth/key"))
        .header("Authorization", auth_header(api_key))
        .send()
        .await
        .map_err(|e| AppError::NetworkError(format!("Request failed: {e}")))?;

    let text = handle_response(resp).await?;
    let parsed: serde_json::Value =
        serde_json::from_str(&text).map_err(|e| AppError::NetworkError(format!("Invalid JSON: {e}")))?;

    let data = parsed
        .get("data")
        .ok_or_else(|| AppError::OpenRouterError("Missing 'data' field in response".into()))?;

    let key_data: CurrentKeyData =
        serde_json::from_value(data.clone()).map_err(|e| AppError::OpenRouterError(format!("Failed to parse key data: {e}")))?;

    Ok(key_data)
}

pub async fn get_credits(api_key: &str) -> AppResult<CreditsResponse> {
    let client = build_client()?;
    let resp = client
        .get(format!("{BASE_URL}/api/v1/credits"))
        .header("Authorization", auth_header(api_key))
        .send()
        .await
        .map_err(|e| AppError::NetworkError(format!("Request failed: {e}")))?;

    let text = handle_response(resp).await?;
    let parsed: CreditsResponse =
        serde_json::from_str(&text).map_err(|e| AppError::OpenRouterError(format!("Failed to parse credits response: {e}")))?;

    Ok(parsed)
}

pub async fn get_keys(api_key: &str) -> AppResult<KeysResponse> {
    let client = build_client()?;
    let resp = client
        .get(format!("{BASE_URL}/api/v1/keys"))
        .header("Authorization", auth_header(api_key))
        .send()
        .await
        .map_err(|e| AppError::NetworkError(format!("Request failed: {e}")))?;

    let text = handle_response(resp).await?;
    let parsed: KeysResponse =
        serde_json::from_str(&text).map_err(|e| AppError::OpenRouterError(format!("Failed to parse keys response: {e}")))?;

    Ok(parsed)
}

pub async fn get_activity(api_key: &str) -> AppResult<ActivityResponse> {
    let client = build_client()?;
    let resp = client
        .get(format!("{BASE_URL}/api/v1/activity"))
        .header("Authorization", auth_header(api_key))
        .send()
        .await
        .map_err(|e| AppError::NetworkError(format!("Request failed: {e}")))?;

    let text = handle_response(resp).await?;
    let parsed: ActivityResponse =
        serde_json::from_str(&text).map_err(|e| AppError::OpenRouterError(format!("Failed to parse activity response: {e}")))?;

    Ok(parsed)
}
