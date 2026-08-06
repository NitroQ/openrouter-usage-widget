use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::error::{AppError, AppResult};
use super::client;
use super::models::{ActivityRow, CurrentKeyData};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryMetric {
    pub label: String,
    pub value: Option<f64>,
    pub unlimited: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageSummary {
    pub today: Option<f64>,
    pub week: Option<f64>,
    pub month: Option<f64>,
    pub total: Option<f64>,
    pub byok_today: Option<f64>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountSummary {
    pub total_credits: Option<f64>,
    pub total_usage: Option<f64>,
    pub remaining_credits: Option<f64>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeysSummary {
    pub total: i64,
    pub active: i64,
    pub disabled: i64,
    pub near_limit: i64,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HistorySummary {
    pub timezone: String,
    pub today_is_provisional: bool,
    pub available_days: i64,
    pub latest: Vec<DailyUsagePoint>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardData {
    pub mode: String,
    pub status: String,
    pub primary_metric: PrimaryMetric,
    pub usage: UsageSummary,
    pub account: Option<AccountSummary>,
    pub keys: Option<KeysSummary>,
    pub limit: Option<f64>,
    pub limit_remaining: Option<f64>,
    pub history: HistorySummary,
    pub refreshed_at: String,
    pub data_source: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyUsagePoint {
    pub date_utc: String,
    pub usage: f64,
    pub byok_usage: f64,
    pub prompt_tokens: i64,
    pub completion_tokens: i64,
    pub reasoning_tokens: i64,
    pub requests: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValidationResult {
    #[serde(rename = "success")]
    pub valid: bool,
    #[serde(rename = "message", serialize_with = "serialize_validation_message")]
    pub error: Option<String>,
    #[serde(rename = "detectedMode")]
    pub mode: Option<String>,
    pub label: Option<String>,
}

fn serialize_validation_message<S: serde::Serializer>(
    val: &Option<String>,
    s: S,
) -> Result<S::Ok, S::Error> {
    match val {
        Some(msg) => s.serialize_some(msg),
        None => s.serialize_some(""),
    }
}

pub fn key_data_to_dashboard(data: &CurrentKeyData, mode: &str) -> DashboardData {
    let now = Utc::now();

    // Determine primary metric
    // Note: total_credits is only available from the credits API (management mode).
    // key_data_to_dashboard only has CurrentKeyData, so credits are None here.
    let (primary_label, primary_value, unlimited) = match mode {
        "management" => (
            "Credits remaining".to_string(),
            data.usage, // Will be overridden by management.rs with actual credits data
            false,
        ),
        _ => {
            if data.limit.is_none() {
                ("Key usage (today)".to_string(), data.usage_daily, false)
            } else {
                (
                    "Key limit remaining".to_string(),
                    data.limit_remaining,
                    false,
                )
            }
        }
    };

    DashboardData {
        mode: mode.to_string(),
        status: "live".to_string(),
        primary_metric: PrimaryMetric {
            label: primary_label,
            value: primary_value,
            unlimited,
        },
        usage: UsageSummary {
            today: data.usage_daily,
            week: data.usage_weekly,
            month: data.usage_monthly,
            total: data.usage,
            byok_today: data.byok_usage_daily,
        },
        account: if mode == "management" {
            Some(AccountSummary {
                total_credits: None, // Populated later by fetch_management_data
                total_usage: data.usage,
                remaining_credits: None, // Populated later by fetch_management_data
            })
        } else {
            None
        },
        keys: None,
        limit: data.limit,
        limit_remaining: data.limit_remaining,
        history: HistorySummary {
            timezone: "UTC".to_string(),
            today_is_provisional: false,
            available_days: 0,
            latest: vec![],
        },
        refreshed_at: now.to_rfc3339(),
        data_source: "network".to_string(),
    }
}

pub async fn fetch_standard_data(api_key: &str) -> AppResult<DashboardData> {
    let key_data = client::get_key_info(api_key).await?;

    if key_data.is_management_key == Some(true) {
        return Err(AppError::InvalidInput(
            "This key appears to be a management key. Please use management mode.".into(),
        ));
    }

    let mut dashboard = key_data_to_dashboard(&key_data, "standard");

    // Fetch activity for standard key
    match client::get_activity(api_key).await {
        Ok(activity_resp) => {
            let points = aggregate_activity(&activity_resp.data);
            dashboard.history.latest = points;
            dashboard.history.available_days = dashboard.history.latest.len() as i64;
        }
        Err(_) => {}
    }

    Ok(dashboard)
}

pub async fn validate_standard_key(api_key: &str) -> AppResult<ValidationResult> {
    let key_data = client::get_key_info(api_key).await?;

    if key_data.is_management_key == Some(true) {
        return Ok(ValidationResult {
            valid: false,
            mode: Some("standard".into()),
            label: key_data.label.clone(),
            error: Some("This key is a management key, not a standard key.".into()),
        });
    }

    Ok(ValidationResult {
        valid: true,
        mode: Some("standard".into()),
        label: key_data.label.clone(),
        error: None,
    })
}

fn aggregate_activity(rows: &[ActivityRow]) -> Vec<DailyUsagePoint> {
    let mut map: std::collections::HashMap<String, DailyUsagePoint> = std::collections::HashMap::new();

    for row in rows {
        let date = row.date.clone().unwrap_or_default();
        let entry = map.entry(date.clone()).or_insert_with(|| DailyUsagePoint {
            date_utc: date,
            usage: 0.0,
            byok_usage: 0.0,
            prompt_tokens: 0,
            completion_tokens: 0,
            reasoning_tokens: 0,
            requests: 0,
        });

        entry.usage += row.usage.unwrap_or(0.0);
        entry.byok_usage += row.byok_usage_inference.unwrap_or(0.0);
        entry.prompt_tokens += row.prompt_tokens.unwrap_or(0);
        entry.completion_tokens += row.completion_tokens.unwrap_or(0);
        entry.reasoning_tokens += row.reasoning_tokens.unwrap_or(0);
        entry.requests += row.requests.unwrap_or(0);
    }

    let mut points: Vec<DailyUsagePoint> = map.into_values().collect();
    points.sort_by(|a, b| a.date_utc.cmp(&b.date_utc));
    points
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::openrouter::models::ActivityRow;

    fn make_row(
        date: &str,
        usage: f64,
        prompt_tokens: i64,
        completion_tokens: i64,
        requests: i64,
    ) -> ActivityRow {
        ActivityRow {
            date: Some(date.to_string()),
            model: Some("test/model".to_string()),
            provider_name: Some("Test".to_string()),
            endpoint_id: Some("ep-1".to_string()),
            prompt_tokens: Some(prompt_tokens),
            completion_tokens: Some(completion_tokens),
            reasoning_tokens: Some(0),
            requests: Some(requests),
            usage: Some(usage),
            byok_usage_inference: Some(0.0),
        }
    }

    #[test]
    fn aggregate_single_date() {
        let rows = vec![
            make_row("2026-08-05", 0.25, 1000, 500, 10),
            make_row("2026-08-05", 0.15, 500, 200, 5),
        ];
        let points = aggregate_activity(&rows);
        assert_eq!(points.len(), 1);
        assert_eq!(points[0].usage, 0.40);
        assert_eq!(points[0].prompt_tokens, 1500);
        assert_eq!(points[0].completion_tokens, 700);
        assert_eq!(points[0].requests, 15);
    }

    #[test]
    fn aggregate_multiple_dates_sorted() {
        let rows = vec![
            make_row("2026-08-07", 0.50, 2000, 1000, 20),
            make_row("2026-08-05", 0.25, 1000, 500, 10),
            make_row("2026-08-06", 0.30, 1500, 700, 15),
        ];
        let points = aggregate_activity(&rows);
        assert_eq!(points.len(), 3);
        assert_eq!(points[0].date_utc, "2026-08-05");
        assert_eq!(points[1].date_utc, "2026-08-06");
        assert_eq!(points[2].date_utc, "2026-08-07");
    }

    #[test]
    fn aggregate_empty_rows() {
        let points = aggregate_activity(&[]);
        assert!(points.is_empty());
    }

    #[test]
    fn aggregate_with_none_values() {
        let rows = vec![ActivityRow {
            date: Some("2026-08-05".to_string()),
            model: None,
            provider_name: None,
            endpoint_id: None,
            prompt_tokens: None,
            completion_tokens: None,
            reasoning_tokens: None,
            requests: None,
            usage: None,
            byok_usage_inference: None,
        }];
        let points = aggregate_activity(&rows);
        assert_eq!(points.len(), 1);
        assert_eq!(points[0].usage, 0.0);
        assert_eq!(points[0].prompt_tokens, 0);
    }

    #[test]
    fn aggregate_with_empty_date() {
        let rows = vec![ActivityRow {
            date: None,
            model: None,
            provider_name: None,
            endpoint_id: None,
            prompt_tokens: Some(100),
            completion_tokens: Some(50),
            reasoning_tokens: None,
            requests: Some(5),
            usage: Some(0.10),
            byok_usage_inference: Some(0.02),
        }];
        let points = aggregate_activity(&rows);
        assert_eq!(points.len(), 1);
        assert_eq!(points[0].date_utc, "");
        assert_eq!(points[0].byok_usage, 0.02);
    }

    #[test]
    fn key_data_to_dashboard_standard() {
        let data = crate::openrouter::models::CurrentKeyData {
            label: Some("my-key".into()),
            limit: Some(100.0),
            limit_remaining: Some(74.5),
            limit_reset: Some("daily".into()),
            usage: Some(25.5),
            usage_daily: Some(1.25),
            usage_weekly: Some(10.0),
            usage_monthly: Some(25.5),
            byok_usage: None,
            byok_usage_daily: Some(0.05),
            byok_usage_weekly: None,
            byok_usage_monthly: None,
            is_management_key: Some(false),
            expires_at: None,
        };
        let dashboard = key_data_to_dashboard(&data, "standard");
        assert_eq!(dashboard.mode, "standard");
        assert_eq!(dashboard.status, "live");
        assert_eq!(dashboard.primary_metric.label, "Key limit remaining");
        assert_eq!(dashboard.primary_metric.value, Some(74.5));
        assert!(!dashboard.primary_metric.unlimited);
        assert_eq!(dashboard.usage.today, Some(1.25));
        assert_eq!(dashboard.usage.byok_today, Some(0.05));
        assert_eq!(dashboard.usage.total, Some(25.5));
        assert!(dashboard.account.is_none());
    }

    #[test]
    fn key_data_to_dashboard_no_limit() {
        let data = crate::openrouter::models::CurrentKeyData {
            label: Some("no-limit".into()),
            limit: None,
            limit_remaining: None,
            limit_reset: None,
            usage: Some(5.0),
            usage_daily: Some(2.0),
            usage_weekly: Some(3.0),
            usage_monthly: Some(5.0),
            byok_usage: None,
            byok_usage_daily: None,
            byok_usage_weekly: None,
            byok_usage_monthly: None,
            is_management_key: Some(false),
            expires_at: None,
        };
        let dashboard = key_data_to_dashboard(&data, "standard");
        assert_eq!(dashboard.primary_metric.label, "Key usage (today)");
        assert_eq!(dashboard.primary_metric.value, Some(2.0));
    }

    #[test]
    fn validation_result_serialization() {
        let result = ValidationResult {
            valid: true,
            mode: Some("standard".into()),
            label: Some("test-key".into()),
            error: None,
        };
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("success"));
        assert!(json.contains("true"));
        assert!(json.contains("detectedMode"));
        assert!(json.contains("standard"));
    }

    #[test]
    fn daily_usage_point_serialization() {
        let point = DailyUsagePoint {
            date_utc: "2026-08-05".into(),
            usage: 0.25,
            byok_usage: 0.0,
            prompt_tokens: 1000,
            completion_tokens: 500,
            reasoning_tokens: 200,
            requests: 10,
        };
        let json = serde_json::to_string(&point).unwrap();
        // Should serialize to camelCase
        assert!(json.contains("dateUtc"));
        assert!(json.contains("2026-08-05"));
        assert!(json.contains("byokUsage"));
        assert!(json.contains("promptTokens"));
        assert!(json.contains("completionTokens"));
        assert!(json.contains("reasoningTokens"));
    }

    #[test]
    fn dashboard_data_serialization_camel_case() {
        let data = crate::openrouter::models::CurrentKeyData {
            label: Some("key".into()),
            limit: None,
            limit_remaining: None,
            limit_reset: None,
            usage: Some(10.0),
            usage_daily: Some(1.0),
            usage_weekly: Some(5.0),
            usage_monthly: Some(10.0),
            byok_usage: None,
            byok_usage_daily: None,
            byok_usage_weekly: None,
            byok_usage_monthly: None,
            is_management_key: Some(false),
            expires_at: None,
        };
        let dashboard = key_data_to_dashboard(&data, "standard");
        let json = serde_json::to_string(&dashboard).unwrap();
        // Verify camelCase serialization
        assert!(json.contains("primaryMetric"));
        assert!(json.contains("refreshedAt"));
        assert!(json.contains("dataSource"));
        assert!(json.contains("byokToday"));
        assert!(json.contains("todayIsProvisional"));
        assert!(json.contains("availableDays"));
        assert!(json.contains("standard"));
    }

    #[test]
    fn dashboard_data_management_account() {
        let data = crate::openrouter::models::CurrentKeyData {
            label: Some("mgmt".into()),
            limit: None,
            limit_remaining: None,
            limit_reset: None,
            usage: Some(25.0),
            usage_daily: Some(1.5),
            usage_weekly: Some(8.0),
            usage_monthly: Some(25.0),
            byok_usage: None,
            byok_usage_daily: None,
            byok_usage_weekly: None,
            byok_usage_monthly: None,
            is_management_key: Some(true),
            expires_at: None,
        };
        let dashboard = key_data_to_dashboard(&data, "management");
        assert_eq!(dashboard.mode, "management");
        assert!(dashboard.account.is_some());
        let account = dashboard.account.unwrap();
        // Credits are None here because key_data_to_dashboard only has CurrentKeyData
        assert_eq!(account.total_credits, None);
        assert_eq!(account.total_usage, Some(25.0));
        assert_eq!(account.remaining_credits, None);
    }
}
