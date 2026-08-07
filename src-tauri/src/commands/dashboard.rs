use tauri::State;

use crate::error::{AppError, AppResult};
use crate::openrouter::standard::{
    AccountSummary, DashboardData, HistorySummary, PrimaryMetric,
    UsageSummary,
};
use crate::storage::credentials as cred_store;
use crate::storage::database::Database;

#[tauri::command]
pub async fn refresh_dashboard(db: State<'_, Database>) -> AppResult<DashboardData> {
    let api_key = cred_store::load_credential()
        .ok_or_else(|| AppError::AuthError("No API key found. Please add a credential first.".into()))?;

    let profile = db.get_active_credential_profile()?
        .ok_or_else(|| AppError::StorageError("No active credential profile found.".into()))?;

    let (profile_id, mode, _fingerprint, _label) = profile;

    let dashboard = match mode.as_str() {
        "standard" => {
            crate::openrouter::standard::fetch_standard_data(&api_key).await?
        }
        "management" => {
            let mut data = crate::openrouter::management::fetch_management_data(&api_key).await?;
            // Also fetch activity for management keys
            match crate::openrouter::management::fetch_activity(&api_key).await {
                Ok(points) => {
                    data.history.available_days = points.len() as i64;
                    data.history.latest = points;
                }
                Err(_) => {}
            }
            data
        }
        _ => {
            return Err(AppError::StorageError(format!(
                "Unknown credential mode: {mode}"
            )));
        }
    };

    // Save snapshot to database - extract flat values from nested structures
    let tracking_date = &dashboard.refreshed_at;
    db.insert_refresh_snapshot(
        profile_id,
        tracking_date,
        dashboard.account.as_ref().and_then(|a| a.total_credits),
        dashboard.account.as_ref().and_then(|a| a.total_usage),
        dashboard.account.as_ref().and_then(|a| a.remaining_credits),
        None, // key_limit not in new structure
        None, // key_limit_remaining not in new structure
        dashboard.usage.today,
        dashboard.usage.week,
        dashboard.usage.month,
        dashboard.usage.total,
        dashboard.usage.byok_today,
        true, // request_succeeded always true after successful fetch
    )?;

    // Save daily usage points
    for point in &dashboard.history.latest {
        let (source, finality) = match mode.as_str() {
            "management" => ("openrouter_activity", "authoritative"),
            _ => ("standard_key_snapshot", "last_seen"),
        };
        db.upsert_daily_usage(
            profile_id,
            &point.date_utc,
            point.usage,
            point.byok_usage,
            point.prompt_tokens,
            point.completion_tokens,
            point.reasoning_tokens,
            point.requests,
            source,
            finality,
        )?;
    }

    if mode == "standard" {
        if let Some(today) = dashboard.usage.today {
            let today_utc = chrono::Utc::now().format("%Y-%m-%d").to_string();
            if !dashboard.history.latest.iter().any(|point| point.date_utc == today_utc) {
                db.upsert_daily_usage(
                    profile_id,
                    &today_utc,
                    today,
                    dashboard.usage.byok_today.unwrap_or(0.0),
                    0,
                    0,
                    0,
                    0,
                    "standard_key_snapshot",
                    "last_seen",
                )?;
            }
        }
    }

    // Fetch and save activity details for management keys
    if mode == "management" {
        if let Ok(activity_resp) = crate::openrouter::client::get_activity(&api_key).await {
            // Group by date and insert
            let mut by_date: std::collections::HashMap<String, Vec<crate::openrouter::models::ActivityRow>> =
                std::collections::HashMap::new();
            for row in activity_resp.data {
                let date = row.date.clone().unwrap_or_default();
                by_date.entry(date).or_default().push(row);
            }
            for (date, rows) in &by_date {
                let _ = db.insert_activity_details(profile_id, date, rows);
            }
        }
    }

    // Prune expired snapshots based on retention setting
    let settings = crate::storage::settings::get_settings().unwrap_or_default();
    if settings.history_retention_days > 0 {
        let _ = db.delete_expired_snapshots(profile_id, settings.history_retention_days as i32);
    }

    Ok(dashboard)
}

#[tauri::command]
pub async fn get_cached_dashboard(db: State<'_, Database>) -> AppResult<Option<DashboardData>> {
    let profile = db.get_active_credential_profile()?
        .ok_or_else(|| AppError::StorageError("No active credential profile found.".into()))?;

    let (profile_id, mode, _fingerprint, _label) = profile;

    // Get recent daily usage
    let daily_usage_points = db.get_daily_usage(profile_id, 30)?;

    // Get last snapshot for the dashboard summary
    let snapshots = db.get_refresh_snapshots(profile_id, 1)?;

    if snapshots.is_empty() {
        return Ok(None);
    }

    let snapshot = &snapshots[0];

    // Extract flat values from snapshot
    let total_credits = snapshot.get("total_credits").and_then(|v| v.as_f64());
    let total_usage = snapshot.get("total_usage").and_then(|v| v.as_f64());
    let credits_remaining = snapshot.get("credits_remaining").and_then(|v| v.as_f64());
    let usage_daily = snapshot.get("usage_daily").and_then(|v| v.as_f64());
    let usage_weekly = snapshot.get("usage_weekly").and_then(|v| v.as_f64());
    let usage_monthly = snapshot.get("usage_monthly").and_then(|v| v.as_f64());
    let usage_all_time = snapshot.get("usage_all_time").and_then(|v| v.as_f64());
    let byok_usage_daily = snapshot.get("byok_usage_daily").and_then(|v| v.as_f64());
    let key_limit = snapshot.get("key_limit").and_then(|v| v.as_f64());
    let key_limit_remaining = snapshot.get("key_limit_remaining").and_then(|v| v.as_f64());
    let tracking_date = snapshot
        .get("tracking_date_utc")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    // Build primary metric based on mode
    let (primary_label, primary_value, unlimited) = if mode == "management" {
        (
            "Credits remaining".to_string(),
            credits_remaining,
            false,
        )
    } else {
        (
            "Key usage (today)".to_string(),
            usage_daily,
            false,
        )
    };

    let is_management = mode == "management";

    let dashboard = DashboardData {
        mode,
        status: "cached".to_string(),
        primary_metric: PrimaryMetric {
            label: primary_label,
            value: primary_value,
            unlimited,
        },
        usage: UsageSummary {
            today: usage_daily,
            week: usage_weekly,
            month: usage_monthly,
            total: usage_all_time,
            byok_today: byok_usage_daily,
        },
        account: if is_management {
            Some(AccountSummary {
                total_credits,
                total_usage,
                remaining_credits: credits_remaining,
            })
        } else {
            None
        },
        keys: None,
        limit: key_limit,
        limit_remaining: key_limit_remaining,
        history: HistorySummary {
            timezone: "UTC".to_string(),
            today_is_provisional: false,
            available_days: daily_usage_points.len() as i64,
            latest: daily_usage_points,
        },
        refreshed_at: tracking_date,
        data_source: "cache".to_string(),
    };

    Ok(Some(dashboard))
}
