use std::sync::Mutex;
use rusqlite::{params, Connection};
use crate::error::{AppError, AppResult};
use crate::openrouter::standard::DailyUsagePoint;
use crate::openrouter::models::ActivityRow;

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn new(conn: Connection) -> Self {
        Self {
            conn: Mutex::new(conn),
        }
    }

    pub fn insert_refresh_snapshot(
        &self,
        credential_profile_id: i64,
        tracking_date_utc: &str,
        total_credits: Option<f64>,
        total_usage: Option<f64>,
        credits_remaining: Option<f64>,
        key_limit: Option<f64>,
        key_limit_remaining: Option<f64>,
        usage_daily: Option<f64>,
        usage_weekly: Option<f64>,
        usage_monthly: Option<f64>,
        usage_all_time: Option<f64>,
        byok_usage_daily: Option<f64>,
        request_succeeded: bool,
    ) -> AppResult<()> {
        let conn = self.conn.lock().map_err(|e| AppError::StorageError(format!("Lock poisoned: {e}")))?;
        conn.execute(
            "INSERT INTO refresh_snapshots (
                credential_profile_id, tracking_date_utc, tracking_date_local,
                total_credits, total_usage, credits_remaining,
                key_limit, key_limit_remaining, usage_daily, usage_weekly,
                usage_monthly, usage_all_time, byok_usage_daily, request_succeeded
            ) VALUES (?1, ?2, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![
                credential_profile_id,
                tracking_date_utc,
                total_credits,
                total_usage,
                credits_remaining,
                key_limit,
                key_limit_remaining,
                usage_daily,
                usage_weekly,
                usage_monthly,
                usage_all_time,
                byok_usage_daily,
                request_succeeded as i32,
            ],
        )
        .map_err(|e| AppError::StorageError(format!("Failed to insert snapshot: {e}")))?;

        Ok(())
    }

    pub fn upsert_daily_usage(
        &self,
        credential_profile_id: i64,
        date_utc: &str,
        usage: f64,
        byok_usage: f64,
        prompt_tokens: i64,
        completion_tokens: i64,
        reasoning_tokens: i64,
        requests: i64,
        source: &str,
        finality: &str,
    ) -> AppResult<()> {
        let conn = self.conn.lock().map_err(|e| AppError::StorageError(format!("Lock poisoned: {e}")))?;
        conn.execute(
            "INSERT INTO daily_usage (
                credential_profile_id, date_utc, usage, byok_usage,
                prompt_tokens, completion_tokens, reasoning_tokens,
                requests, source, finality, first_refreshed_at_utc, last_refreshed_at_utc, sample_count
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, datetime('now'), datetime('now'), 1)
            ON CONFLICT(credential_profile_id, date_utc) DO UPDATE SET
                usage = ?3,
                byok_usage = ?4,
                prompt_tokens = ?5,
                completion_tokens = ?6,
                reasoning_tokens = ?7,
                requests = ?8,
                source = ?9,
                finality = ?10,
                last_refreshed_at_utc = datetime('now'),
                sample_count = sample_count + 1",
            params![
                credential_profile_id,
                date_utc,
                usage,
                byok_usage,
                prompt_tokens,
                completion_tokens,
                reasoning_tokens,
                requests,
                source,
                finality,
            ],
        )
        .map_err(|e| AppError::StorageError(format!("Failed to upsert daily usage: {e}")))?;

        Ok(())
    }

    pub fn get_daily_usage(&self, credential_profile_id: i64, days: i32) -> AppResult<Vec<DailyUsagePoint>> {
        let conn = self.conn.lock().map_err(|e| AppError::StorageError(format!("Lock poisoned: {e}")))?;
        let mut stmt = conn
            .prepare(
                "SELECT date_utc, usage, byok_usage, prompt_tokens, completion_tokens, reasoning_tokens, requests
                 FROM daily_usage
                 WHERE credential_profile_id = ?1
                   AND date_utc >= date('now', ?2)
                 ORDER BY date_utc ASC",
            )
            .map_err(|e| AppError::StorageError(format!("Failed to prepare query: {e}")))?;

        let offset = format!("-{days} days");
        let rows = stmt
            .query_map(params![credential_profile_id, offset], |row| {
                Ok(DailyUsagePoint {
                    date_utc: row.get(0)?,
                    usage: row.get(1)?,
                    byok_usage: row.get(2)?,
                    prompt_tokens: row.get(3)?,
                    completion_tokens: row.get(4)?,
                    reasoning_tokens: row.get(5)?,
                    requests: row.get(6)?,
                })
            })
            .map_err(|e| AppError::StorageError(format!("Failed to query daily usage: {e}")))?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row.map_err(|e| AppError::StorageError(format!("Failed to read row: {e}")))?);
        }

        Ok(results)
    }

    pub fn get_daily_usage_range(
        &self,
        credential_profile_id: i64,
        start_date: &str,
        end_date: &str,
    ) -> AppResult<Vec<DailyUsagePoint>> {
        let conn = self.conn.lock().map_err(|e| AppError::StorageError(format!("Lock poisoned: {e}")))?;
        let mut stmt = conn
            .prepare(
                "SELECT date_utc, usage, byok_usage, prompt_tokens, completion_tokens, reasoning_tokens, requests
                 FROM daily_usage
                 WHERE credential_profile_id = ?1
                   AND date_utc >= ?2
                   AND date_utc <= ?3
                 ORDER BY date_utc ASC",
            )
            .map_err(|e| AppError::StorageError(format!("Failed to prepare query: {e}")))?;

        let rows = stmt
            .query_map(params![credential_profile_id, start_date, end_date], |row| {
                Ok(DailyUsagePoint {
                    date_utc: row.get(0)?,
                    usage: row.get(1)?,
                    byok_usage: row.get(2)?,
                    prompt_tokens: row.get(3)?,
                    completion_tokens: row.get(4)?,
                    reasoning_tokens: row.get(5)?,
                    requests: row.get(6)?,
                })
            })
            .map_err(|e| AppError::StorageError(format!("Failed to query daily usage: {e}")))?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row.map_err(|e| AppError::StorageError(format!("Failed to read row: {e}")))?);
        }

        Ok(results)
    }

    pub fn get_refresh_snapshots(&self, credential_profile_id: i64, limit: i32) -> AppResult<Vec<serde_json::Value>> {
        let conn = self.conn.lock().map_err(|e| AppError::StorageError(format!("Lock poisoned: {e}")))?;
        let mut stmt = conn
            .prepare(
                "SELECT id, refreshed_at_utc, tracking_date_utc, total_credits, total_usage,
                        credits_remaining, key_limit, key_limit_remaining, usage_daily, usage_weekly,
                        usage_monthly, usage_all_time, byok_usage_daily, request_succeeded
                 FROM refresh_snapshots
                 WHERE credential_profile_id = ?1
                 ORDER BY id DESC
                 LIMIT ?2",
            )
            .map_err(|e| AppError::StorageError(format!("Failed to prepare query: {e}")))?;

        let rows = stmt
            .query_map(params![credential_profile_id, limit], |row| {
                Ok(serde_json::json!({
                    "id": row.get::<_, i64>(0)?,
                    "refreshed_at_utc": row.get::<_, String>(1)?,
                    "tracking_date_utc": row.get::<_, Option<String>>(2)?,
                    "total_credits": row.get::<_, Option<f64>>(3)?,
                    "total_usage": row.get::<_, Option<f64>>(4)?,
                    "credits_remaining": row.get::<_, Option<f64>>(5)?,
                    "key_limit": row.get::<_, Option<f64>>(6)?,
                    "key_limit_remaining": row.get::<_, Option<f64>>(7)?,
                    "usage_daily": row.get::<_, Option<f64>>(8)?,
                    "usage_weekly": row.get::<_, Option<f64>>(9)?,
                    "usage_monthly": row.get::<_, Option<f64>>(10)?,
                    "usage_all_time": row.get::<_, Option<f64>>(11)?,
                    "byok_usage_daily": row.get::<_, Option<f64>>(12)?,
                    "request_succeeded": row.get::<_, bool>(13)?,
                }))
            })
            .map_err(|e| AppError::StorageError(format!("Failed to query snapshots: {e}")))?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row.map_err(|e| AppError::StorageError(format!("Failed to read row: {e}")))?);
        }

        Ok(results)
    }

    pub fn delete_expired_snapshots(&self, credential_profile_id: i64, days: i32) -> AppResult<()> {
        let conn = self.conn.lock().map_err(|e| AppError::StorageError(format!("Lock poisoned: {e}")))?;
        let offset = format!("-{days} days");
        conn.execute(
            "DELETE FROM refresh_snapshots
             WHERE credential_profile_id = ?1
               AND tracking_date_utc < date('now', ?2)",
            params![credential_profile_id, offset],
        )
        .map_err(|e| AppError::StorageError(format!("Failed to delete expired snapshots: {e}")))?;

        Ok(())
    }

    pub fn clear_history(&self, credential_profile_id: i64) -> AppResult<()> {
        let conn = self.conn.lock().map_err(|e| AppError::StorageError(format!("Lock poisoned: {e}")))?;
        conn.execute(
            "DELETE FROM daily_usage WHERE credential_profile_id = ?1",
            params![credential_profile_id],
        )
        .map_err(|e| AppError::StorageError(format!("Failed to clear daily usage: {e}")))?;

        conn.execute(
            "DELETE FROM refresh_snapshots WHERE credential_profile_id = ?1",
            params![credential_profile_id],
        )
        .map_err(|e| AppError::StorageError(format!("Failed to clear snapshots: {e}")))?;

        conn.execute(
            "DELETE FROM daily_activity_details WHERE credential_profile_id = ?1",
            params![credential_profile_id],
        )
        .map_err(|e| AppError::StorageError(format!("Failed to clear activity details: {e}")))?;

        Ok(())
    }

    pub fn get_all_daily_usage_for_export(&self, credential_profile_id: i64) -> AppResult<Vec<DailyUsagePoint>> {
        let conn = self.conn.lock().map_err(|e| AppError::StorageError(format!("Lock poisoned: {e}")))?;
        let mut stmt = conn
            .prepare(
                "SELECT date_utc, usage, byok_usage, prompt_tokens, completion_tokens, reasoning_tokens, requests
                 FROM daily_usage
                 WHERE credential_profile_id = ?1
                 ORDER BY date_utc ASC",
            )
            .map_err(|e| AppError::StorageError(format!("Failed to prepare query: {e}")))?;

        let rows = stmt
            .query_map(params![credential_profile_id], |row| {
                Ok(DailyUsagePoint {
                    date_utc: row.get(0)?,
                    usage: row.get(1)?,
                    byok_usage: row.get(2)?,
                    prompt_tokens: row.get(3)?,
                    completion_tokens: row.get(4)?,
                    reasoning_tokens: row.get(5)?,
                    requests: row.get(6)?,
                })
            })
            .map_err(|e| AppError::StorageError(format!("Failed to query usage: {e}")))?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row.map_err(|e| AppError::StorageError(format!("Failed to read row: {e}")))?);
        }

        Ok(results)
    }

    pub fn insert_activity_details(
        &self,
        credential_profile_id: i64,
        date_utc: &str,
        rows: &[ActivityRow],
    ) -> AppResult<()> {
        let conn = self.conn.lock().map_err(|e| AppError::StorageError(format!("Lock poisoned: {e}")))?;

        for row in rows {
            conn.execute(
                "INSERT OR REPLACE INTO daily_activity_details (
                    credential_profile_id, date_utc, model, provider_name, endpoint_id,
                    usage, byok_usage, prompt_tokens, completion_tokens, reasoning_tokens, requests
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                params![
                    credential_profile_id,
                    date_utc,
                    row.model.as_deref().unwrap_or("unknown"),
                    row.provider_name.as_deref().unwrap_or(""),
                    row.endpoint_id.as_deref().unwrap_or(""),
                    row.usage.unwrap_or(0.0),
                    row.byok_usage_inference.unwrap_or(0.0),
                    row.prompt_tokens.unwrap_or(0),
                    row.completion_tokens.unwrap_or(0),
                    row.reasoning_tokens.unwrap_or(0),
                    row.requests.unwrap_or(0),
                ],
            )
            .map_err(|e| AppError::StorageError(format!("Failed to insert activity detail: {e}")))?;
        }

        Ok(())
    }

    pub fn create_credential_profile(
        &self,
        mode: &str,
        key_fingerprint: &str,
        label: Option<&str>,
    ) -> AppResult<i64> {
        let conn = self.conn.lock().map_err(|e| AppError::StorageError(format!("Lock poisoned: {e}")))?;

        // Deactivate all existing profiles
        conn.execute(
            "UPDATE credential_profiles SET is_active = 0 WHERE is_active = 1",
            [],
        )
        .map_err(|e| AppError::StorageError(format!("Failed to deactivate profiles: {e}")))?;

        conn.execute(
            "INSERT INTO credential_profiles (mode, key_fingerprint, label, is_active)
             VALUES (?1, ?2, ?3, 1)",
            params![mode, key_fingerprint, label],
        )
        .map_err(|e| AppError::StorageError(format!("Failed to create credential profile: {e}")))?;

        let id = conn.last_insert_rowid();
        Ok(id)
    }

    pub fn get_active_credential_profile(&self) -> AppResult<Option<(i64, String, String, Option<String>)>> {
        let conn = self.conn.lock().map_err(|e| AppError::StorageError(format!("Lock poisoned: {e}")))?;
        let mut stmt = conn
            .prepare(
                "SELECT id, mode, key_fingerprint, label FROM credential_profiles WHERE is_active = 1 LIMIT 1",
            )
            .map_err(|e| AppError::StorageError(format!("Failed to prepare query: {e}")))?;

        let mut rows = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, Option<String>>(3)?,
                ))
            })
            .map_err(|e| AppError::StorageError(format!("Failed to query profiles: {e}")))?;

        match rows.next() {
            Some(row) => {
                let data = row.map_err(|e| AppError::StorageError(format!("Failed to read row: {e}")))?;
                Ok(Some(data))
            }
            None => Ok(None),
        }
    }

    pub fn deactivate_all_profiles(&self) -> AppResult<()> {
        let conn = self.conn.lock().map_err(|e| AppError::StorageError(format!("Lock poisoned: {e}")))?;
        conn.execute("UPDATE credential_profiles SET is_active = 0", [])
            .map_err(|e| AppError::StorageError(format!("Failed to deactivate profiles: {e}")))?;
        Ok(())
    }

    pub fn clear_all_data(&self) -> AppResult<()> {
        let conn = self.conn.lock().map_err(|e| AppError::StorageError(format!("Lock poisoned: {e}")))?;
        for table in ["daily_activity_details", "daily_usage", "refresh_snapshots", "credential_profiles"] {
            conn.execute(&format!("DELETE FROM {table}"), [])
                .map_err(|e| AppError::StorageError(format!("Failed to clear {table}: {e}")))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn setup_db() -> Database {
        let conn = Connection::open_in_memory().unwrap();
        let migration_sql = include_str!("../../migrations/001_initial.sql");
        conn.execute_batch(migration_sql).unwrap();
        Database::new(conn)
    }

    #[test]
    fn create_credential_profile() {
        let db = setup_db();
        let id = db.create_credential_profile("standard", "fp123", Some("test-key")).unwrap();
        assert!(id > 0);
    }

    #[test]
    fn get_active_credential_profile() {
        let db = setup_db();
        db.create_credential_profile("standard", "fp123", Some("test-key")).unwrap();
        let profile = db.get_active_credential_profile().unwrap();
        assert!(profile.is_some());
        let (id, mode, fingerprint, label) = profile.unwrap();
        assert_eq!(id, 1);
        assert_eq!(mode, "standard");
        assert_eq!(fingerprint, "fp123");
        assert_eq!(label.as_deref(), Some("test-key"));
    }

    #[test]
    fn get_active_profile_none_when_empty() {
        let db = setup_db();
        let profile = db.get_active_credential_profile().unwrap();
        assert!(profile.is_none());
    }

    #[test]
    fn create_profile_deactivates_old() {
        let db = setup_db();
        db.create_credential_profile("standard", "fp1", Some("key1")).unwrap();
        db.create_credential_profile("management", "fp2", Some("key2")).unwrap();

        let profile = db.get_active_credential_profile().unwrap().unwrap();
        assert_eq!(profile.1, "management");
        assert_eq!(profile.2, "fp2");
    }

    #[test]
    fn deactivate_all_profiles() {
        let db = setup_db();
        db.create_credential_profile("standard", "fp1", None).unwrap();
        db.deactivate_all_profiles().unwrap();
        let profile = db.get_active_credential_profile().unwrap();
        assert!(profile.is_none());
    }

    #[test]
    fn clear_all_data_removes_profiles_and_history() {
        let db = setup_db();
        let profile_id = db.create_credential_profile("standard", "fp1", None).unwrap();
        db.upsert_daily_usage(profile_id, "2026-08-05", 0.25, 0.0, 1, 1, 0, 1, "standard_key_snapshot", "last_seen").unwrap();

        db.clear_all_data().unwrap();

        assert!(db.get_active_credential_profile().unwrap().is_none());
        assert!(db.get_daily_usage(profile_id, 365).unwrap().is_empty());
    }

    #[test]
    fn insert_and_get_refresh_snapshot() {
        let db = setup_db();
        let profile_id = db.create_credential_profile("standard", "fp1", None).unwrap();

        db.insert_refresh_snapshot(
            profile_id,
            "2026-08-05",
            Some(100.0),
            Some(25.5),
            Some(74.5),
            Some(100.0),
            Some(74.5),
            Some(1.25),
            Some(10.0),
            Some(25.5),
            Some(25.5),
            Some(0.0),
            true,
        ).unwrap();

        let snapshots = db.get_refresh_snapshots(profile_id, 10).unwrap();
        assert_eq!(snapshots.len(), 1);
        assert_eq!(snapshots[0]["total_credits"], 100.0);
        assert_eq!(snapshots[0]["usage_daily"], 1.25);
        assert_eq!(snapshots[0]["request_succeeded"], true);
    }

    #[test]
    fn upsert_daily_usage_insert() {
        let db = setup_db();
        let profile_id = db.create_credential_profile("standard", "fp1", None).unwrap();

        db.upsert_daily_usage(profile_id, "2026-08-05", 0.25, 0.0, 1000, 500, 200, 10, "standard_key_snapshot", "last_seen").unwrap();

        let points = db.get_daily_usage(profile_id, 365).unwrap();
        assert_eq!(points.len(), 1);
        assert_eq!(points[0].usage, 0.25);
        assert_eq!(points[0].prompt_tokens, 1000);
    }

    #[test]
    fn upsert_daily_usage_update() {
        let db = setup_db();
        let profile_id = db.create_credential_profile("standard", "fp1", None).unwrap();

        db.upsert_daily_usage(profile_id, "2026-08-05", 0.25, 0.0, 1000, 500, 200, 10, "standard_key_snapshot", "last_seen").unwrap();
        db.upsert_daily_usage(profile_id, "2026-08-05", 0.50, 0.0, 2000, 1000, 400, 20, "standard_key_snapshot", "last_seen").unwrap();

        let points = db.get_daily_usage(profile_id, 365).unwrap();
        assert_eq!(points.len(), 1);
        assert_eq!(points[0].usage, 0.50);
        assert_eq!(points[0].prompt_tokens, 2000);
    }

    #[test]
    fn get_daily_usage_range() {
        let db = setup_db();
        let profile_id = db.create_credential_profile("standard", "fp1", None).unwrap();

        db.upsert_daily_usage(profile_id, "2026-08-01", 0.1, 0.0, 100, 50, 10, 5, "standard_key_snapshot", "last_seen").unwrap();
        db.upsert_daily_usage(profile_id, "2026-08-05", 0.5, 0.0, 500, 250, 50, 25, "standard_key_snapshot", "last_seen").unwrap();
        db.upsert_daily_usage(profile_id, "2026-08-10", 1.0, 0.0, 1000, 500, 100, 50, "standard_key_snapshot", "last_seen").unwrap();

        let points = db.get_daily_usage_range(profile_id, "2026-08-03", "2026-08-07").unwrap();
        assert_eq!(points.len(), 1);
        assert_eq!(points[0].date_utc, "2026-08-05");
    }

    #[test]
    fn clear_history() {
        let db = setup_db();
        let profile_id = db.create_credential_profile("standard", "fp1", None).unwrap();

        db.upsert_daily_usage(profile_id, "2026-08-05", 0.25, 0.0, 1000, 500, 200, 10, "standard_key_snapshot", "last_seen").unwrap();
        db.insert_refresh_snapshot(profile_id, "2026-08-05", None, None, None, None, None, None, None, None, None, None, true).unwrap();

        db.clear_history(profile_id).unwrap();

        let points = db.get_daily_usage(profile_id, 365).unwrap();
        assert!(points.is_empty());
        let snapshots = db.get_refresh_snapshots(profile_id, 10).unwrap();
        assert!(snapshots.is_empty());
    }

    #[test]
    fn delete_expired_snapshots() {
        let db = setup_db();
        let profile_id = db.create_credential_profile("standard", "fp1", None).unwrap();

        db.insert_refresh_snapshot(profile_id, "2026-08-05", None, None, None, None, None, None, None, None, None, None, true).unwrap();
        db.insert_refresh_snapshot(profile_id, "2020-01-01", None, None, None, None, None, None, None, None, None, None, true).unwrap();

        db.delete_expired_snapshots(profile_id, 30).unwrap();

        let snapshots = db.get_refresh_snapshots(profile_id, 100).unwrap();
        assert_eq!(snapshots.len(), 1);
    }

    #[test]
    fn get_all_daily_usage_for_export() {
        let db = setup_db();
        let profile_id = db.create_credential_profile("standard", "fp1", None).unwrap();

        db.upsert_daily_usage(profile_id, "2026-08-01", 0.1, 0.0, 100, 50, 10, 5, "standard_key_snapshot", "last_seen").unwrap();
        db.upsert_daily_usage(profile_id, "2026-08-02", 0.2, 0.0, 200, 100, 20, 10, "standard_key_snapshot", "last_seen").unwrap();

        let points = db.get_all_daily_usage_for_export(profile_id).unwrap();
        assert_eq!(points.len(), 2);
        assert_eq!(points[0].date_utc, "2026-08-01");
        assert_eq!(points[1].date_utc, "2026-08-02");
    }

    #[test]
    fn insert_activity_details() {
        let db = setup_db();
        let profile_id = db.create_credential_profile("management", "fp1", None).unwrap();

        let rows = vec![
            ActivityRow {
                date: Some("2026-08-05".into()),
                model: Some("openai/gpt-4.1".into()),
                provider_name: Some("OpenAI".into()),
                endpoint_id: Some("ep-1".into()),
                prompt_tokens: Some(1000),
                completion_tokens: Some(500),
                reasoning_tokens: Some(200),
                requests: Some(10),
                usage: Some(0.25),
                byok_usage_inference: Some(0.0),
            },
        ];

        db.insert_activity_details(profile_id, "2026-08-05", &rows).unwrap();

        let conn = db.conn.lock().unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM daily_activity_details WHERE credential_profile_id = ?1", [profile_id], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);
    }
}
