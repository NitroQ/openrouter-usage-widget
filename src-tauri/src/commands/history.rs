use tauri::State;

use crate::error::{AppError, AppResult};
use crate::openrouter::standard::DailyUsagePoint;
use crate::storage::database::Database;

#[tauri::command]
pub async fn get_usage_history(
    days: i32,
    db: State<'_, Database>,
) -> AppResult<Vec<DailyUsagePoint>> {
    let profile = db.get_active_credential_profile()?
        .ok_or_else(|| AppError::StorageError("No active credential profile found.".into()))?;

    let (profile_id, _, _, _) = profile;

    let points = db.get_daily_usage(profile_id, days)?;
    Ok(points)
}

#[tauri::command]
pub async fn export_usage_history_csv(db: State<'_, Database>) -> AppResult<String> {
    let profile = db.get_active_credential_profile()?
        .ok_or_else(|| AppError::StorageError("No active credential profile found.".into()))?;

    let (profile_id, _, _, _) = profile;

    let points = db.get_all_daily_usage_for_export(profile_id)?;

    let mut csv = String::from("Date,Usage,BYOK Usage,Prompt Tokens,Completion Tokens,Reasoning Tokens,Requests\n");
    for point in &points {
        csv.push_str(&format!(
            "{},{:.6},{:.6},{},{},{},{}\n",
            point.date_utc,
            point.usage,
            point.byok_usage,
            point.prompt_tokens,
            point.completion_tokens,
            point.reasoning_tokens,
            point.requests,
        ));
    }

    Ok(csv)
}

#[tauri::command]
pub async fn clear_usage_history(db: State<'_, Database>) -> AppResult<()> {
    let profile = db.get_active_credential_profile()?
        .ok_or_else(|| AppError::StorageError("No active credential profile found.".into()))?;

    let (profile_id, _, _, _) = profile;

    db.clear_history(profile_id)?;

    Ok(())
}
