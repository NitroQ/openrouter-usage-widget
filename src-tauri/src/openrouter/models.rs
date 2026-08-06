use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentKeyData {
    pub label: Option<String>,
    pub limit: Option<f64>,
    pub limit_remaining: Option<f64>,
    pub limit_reset: Option<String>,
    pub usage: Option<f64>,
    pub usage_daily: Option<f64>,
    pub usage_weekly: Option<f64>,
    pub usage_monthly: Option<f64>,
    pub byok_usage: Option<f64>,
    pub byok_usage_daily: Option<f64>,
    pub byok_usage_weekly: Option<f64>,
    pub byok_usage_monthly: Option<f64>,
    pub is_management_key: Option<bool>,
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditsResponse {
    pub data: CreditsData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditsData {
    pub total_credits: Option<f64>,
    pub total_usage: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeysResponse {
    pub data: Vec<KeyInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyInfo {
    pub key_hash: Option<String>,
    pub label: Option<String>,
    pub limit: Option<f64>,
    pub limit_remaining: Option<f64>,
    pub limit_reset: Option<String>,
    pub usage: Option<f64>,
    pub usage_daily: Option<f64>,
    pub usage_weekly: Option<f64>,
    pub usage_monthly: Option<f64>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityResponse {
    pub data: Vec<ActivityRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityRow {
    pub date: Option<String>,
    pub model: Option<String>,
    pub provider_name: Option<String>,
    pub endpoint_id: Option<String>,
    pub prompt_tokens: Option<i64>,
    pub completion_tokens: Option<i64>,
    pub reasoning_tokens: Option<i64>,
    pub requests: Option<i64>,
    pub usage: Option<f64>,
    pub byok_usage_inference: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_current_key_data() {
        let json = r#"{
            "label": "test-key",
            "limit": 100.0,
            "limit_remaining": 74.5,
            "limit_reset": "daily",
            "usage": 25.5,
            "usage_daily": 1.25,
            "usage_weekly": 10.0,
            "usage_monthly": 25.5,
            "byok_usage": 0.0,
            "byok_usage_daily": 0.0,
            "byok_usage_weekly": 0.0,
            "byok_usage_monthly": 0.0,
            "is_management_key": false,
            "expires_at": null
        }"#;
        let data: CurrentKeyData = serde_json::from_str(json).unwrap();
        assert_eq!(data.label.as_deref(), Some("test-key"));
        assert_eq!(data.limit, Some(100.0));
        assert_eq!(data.limit_remaining, Some(74.5));
        assert_eq!(data.usage, Some(25.5));
        assert_eq!(data.is_management_key, Some(false));
        assert!(data.expires_at.is_none());
    }

    #[test]
    fn deserialize_current_key_data_minimal() {
        let json = r#"{}"#;
        let data: CurrentKeyData = serde_json::from_str(json).unwrap();
        assert!(data.label.is_none());
        assert!(data.limit.is_none());
        assert!(data.is_management_key.is_none());
    }

    #[test]
    fn deserialize_credits_response() {
        let json = r#"{
            "data": {
                "total_credits": 100.5,
                "total_usage": 25.75
            }
        }"#;
        let resp: CreditsResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.data.total_credits, Some(100.5));
        assert_eq!(resp.data.total_usage, Some(25.75));
    }

    #[test]
    fn deserialize_credits_response_null_values() {
        let json = r#"{
            "data": {
                "total_credits": null,
                "total_usage": null
            }
        }"#;
        let resp: CreditsResponse = serde_json::from_str(json).unwrap();
        assert!(resp.data.total_credits.is_none());
        assert!(resp.data.total_usage.is_none());
    }

    #[test]
    fn deserialize_keys_response() {
        let json = r#"{
            "data": [
                {
                    "key_hash": "abc123",
                    "label": "key-1",
                    "limit": 50.0,
                    "limit_remaining": 30.0,
                    "limit_reset": "daily",
                    "usage": 20.0,
                    "usage_daily": 1.0,
                    "usage_weekly": 5.0,
                    "usage_monthly": 20.0,
                    "created_at": "2026-01-01"
                }
            ]
        }"#;
        let resp: KeysResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.data.len(), 1);
        assert_eq!(resp.data[0].label.as_deref(), Some("key-1"));
    }

    #[test]
    fn deserialize_activity_response() {
        let json = r#"{
            "data": [
                {
                    "date": "2026-08-05",
                    "model": "openai/gpt-4.1",
                    "provider_name": "OpenAI",
                    "endpoint_id": "ep-1",
                    "prompt_tokens": 1000,
                    "completion_tokens": 500,
                    "reasoning_tokens": 200,
                    "requests": 10,
                    "usage": 0.25,
                    "byok_usage_inference": 0.0
                }
            ]
        }"#;
        let resp: ActivityResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.data.len(), 1);
        let row = &resp.data[0];
        assert_eq!(row.date.as_deref(), Some("2026-08-05"));
        assert_eq!(row.model.as_deref(), Some("openai/gpt-4.1"));
        assert_eq!(row.prompt_tokens, Some(1000));
        assert_eq!(row.usage, Some(0.25));
    }

    #[test]
    fn serialize_current_key_data() {
        let data = CurrentKeyData {
            label: Some("test".into()),
            limit: Some(100.0),
            limit_remaining: Some(50.0),
            limit_reset: Some("daily".into()),
            usage: Some(50.0),
            usage_daily: Some(5.0),
            usage_weekly: Some(20.0),
            usage_monthly: Some(50.0),
            byok_usage: None,
            byok_usage_daily: None,
            byok_usage_weekly: None,
            byok_usage_monthly: None,
            is_management_key: Some(false),
            expires_at: None,
        };
        let json = serde_json::to_string(&data).unwrap();
        assert!(json.contains("test"));
        assert!(json.contains("100.0"));
    }

    #[test]
    fn empty_keys_data() {
        let json = r#"{"data": []}"#;
        let resp: KeysResponse = serde_json::from_str(json).unwrap();
        assert!(resp.data.is_empty());
    }

    #[test]
    fn empty_activity_data() {
        let json = r#"{"data": []}"#;
        let resp: ActivityResponse = serde_json::from_str(json).unwrap();
        assert!(resp.data.is_empty());
    }
}
