use crate::error::{AppError, AppResult};
use super::client;
use super::models::*;
use super::standard::{
    DashboardData, DailyUsagePoint, KeysSummary, ValidationResult,
    key_data_to_dashboard,
};

pub async fn fetch_management_data(api_key: &str) -> AppResult<DashboardData> {
    let key_data = client::get_key_info(api_key).await?;

    if key_data.is_management_key != Some(true) {
        return Err(AppError::InvalidInput(
            "This key is not a management key. Please use standard mode.".into(),
        ));
    }

    let credits = client::get_credits(api_key).await.ok();
    let keys_resp = client::get_keys(api_key).await.ok();

    let mut dashboard = key_data_to_dashboard(&key_data, "management");

    // Add credits data
    if let Some(credits_data) = credits {
        if let Some(ref mut account) = dashboard.account {
            account.total_credits = credits_data.data.total_credits;
            account.total_usage = credits_data.data.total_usage;
            account.remaining_credits = credits_data.data.total_credits
                .zip(credits_data.data.total_usage)
                .map(|(c, u)| c - u);
        }
    }

    // Update primary metric with actual credits remaining
    if let Some(ref account) = dashboard.account {
        dashboard.primary_metric.value = account.remaining_credits;
    }

    // Add keys summary
    if let Some(keys_data) = keys_resp {
        let total = keys_data.data.len() as i64;
        let active = keys_data.data.iter().filter(|k| k.usage.is_some()).count() as i64;
        dashboard.keys = Some(KeysSummary {
            total,
            active,
            disabled: total - active,
            near_limit: 0,
        });
    }

    dashboard.status = "live".to_string();
    Ok(dashboard)
}

pub async fn validate_management_key(api_key: &str) -> AppResult<ValidationResult> {
    let key_data = client::get_key_info(api_key).await?;

    if key_data.is_management_key != Some(true) {
        return Ok(ValidationResult {
            valid: false,
            mode: Some("management".into()),
            label: key_data.label.clone(),
            error: Some("This key is not a management key.".into()),
        });
    }

    Ok(ValidationResult {
        valid: true,
        mode: Some("management".into()),
        label: key_data.label.clone(),
        error: None,
    })
}

pub async fn fetch_activity(api_key: &str) -> AppResult<Vec<DailyUsagePoint>> {
    let activity_resp = client::get_activity(api_key).await?;
    let points = aggregate_activity(&activity_resp.data);
    Ok(points)
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

    fn make_row(date: &str, usage: f64, requests: i64) -> ActivityRow {
        ActivityRow {
            date: Some(date.to_string()),
            model: Some("test/model".to_string()),
            provider_name: Some("Test".to_string()),
            endpoint_id: Some("ep-1".to_string()),
            prompt_tokens: Some(1000),
            completion_tokens: Some(500),
            reasoning_tokens: Some(100),
            requests: Some(requests),
            usage: Some(usage),
            byok_usage_inference: Some(0.01),
        }
    }

    #[test]
    fn aggregate_management_activity_groups_by_date() {
        let rows = vec![
            make_row("2026-08-05", 0.25, 10),
            make_row("2026-08-05", 0.15, 5),
            make_row("2026-08-06", 0.30, 15),
        ];
        let points = aggregate_activity(&rows);
        assert_eq!(points.len(), 2);
        assert_eq!(points[0].date_utc, "2026-08-05");
        assert!((points[0].usage - 0.40).abs() < 0.001);
        assert_eq!(points[0].requests, 15);
        assert_eq!(points[1].date_utc, "2026-08-06");
    }

    #[test]
    fn aggregate_management_activity_sorted() {
        let rows = vec![
            make_row("2026-08-10", 1.0, 50),
            make_row("2026-08-01", 0.1, 5),
            make_row("2026-08-05", 0.5, 25),
        ];
        let points = aggregate_activity(&rows);
        assert_eq!(points[0].date_utc, "2026-08-01");
        assert_eq!(points[1].date_utc, "2026-08-05");
        assert_eq!(points[2].date_utc, "2026-08-10");
    }

    #[test]
    fn aggregate_management_empty() {
        let points = aggregate_activity(&[]);
        assert!(points.is_empty());
    }

    #[test]
    fn dashboard_data_management_mode() {
        let data = crate::openrouter::models::CurrentKeyData {
            label: Some("mgmt-key".into()),
            limit: None,
            limit_remaining: None,
            limit_reset: None,
            usage: Some(25.75),
            usage_daily: Some(1.25),
            usage_weekly: Some(10.0),
            usage_monthly: Some(25.75),
            byok_usage: None,
            byok_usage_daily: None,
            byok_usage_weekly: None,
            byok_usage_monthly: None,
            is_management_key: Some(true),
            expires_at: None,
        };

        // Test that key_data_to_dashboard works with management mode
        let dashboard = key_data_to_dashboard(&data, "management");
        assert_eq!(dashboard.mode, "management");
        assert!(dashboard.account.is_some());
        let account = dashboard.account.as_ref().unwrap();
        assert_eq!(account.total_usage, Some(25.75));
        assert_eq!(dashboard.primary_metric.label, "Credits remaining");
    }

    #[test]
    fn credit_calculation() {
        let total_credits = Some(100.5);
        let total_usage = Some(25.75);
        let remaining = total_credits.zip(total_usage).map(|(c, u)| c - u);
        assert_eq!(remaining, Some(74.75));
    }

    #[test]
    fn credit_calculation_none_values() {
        let total_credits: Option<f64> = None;
        let total_usage: Option<f64> = None;
        let remaining = total_credits.zip(total_usage).map(|(c, u)| c - u);
        assert!(remaining.is_none());
    }
}
