CREATE TABLE IF NOT EXISTS credential_profiles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    mode TEXT NOT NULL CHECK (mode IN ('standard', 'management')),
    key_fingerprint TEXT NOT NULL,
    label TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    last_validated_at TEXT,
    is_active INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS refresh_snapshots (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    credential_profile_id INTEGER NOT NULL,
    refreshed_at_utc TEXT NOT NULL DEFAULT (datetime('now')),
    tracking_date_utc TEXT,
    tracking_date_local TEXT,
    total_credits REAL,
    total_usage REAL,
    credits_remaining REAL,
    key_limit REAL,
    key_limit_remaining REAL,
    usage_daily REAL,
    usage_weekly REAL,
    usage_monthly REAL,
    usage_all_time REAL,
    byok_usage_daily REAL,
    request_succeeded INTEGER,
    FOREIGN KEY (credential_profile_id) REFERENCES credential_profiles(id)
);

CREATE TABLE IF NOT EXISTS daily_usage (
    credential_profile_id INTEGER NOT NULL,
    date_utc TEXT NOT NULL,
    usage REAL NOT NULL DEFAULT 0,
    byok_usage REAL NOT NULL DEFAULT 0,
    prompt_tokens INTEGER NOT NULL DEFAULT 0,
    completion_tokens INTEGER NOT NULL DEFAULT 0,
    reasoning_tokens INTEGER NOT NULL DEFAULT 0,
    requests INTEGER NOT NULL DEFAULT 0,
    source TEXT NOT NULL CHECK (source IN ('openrouter_activity', 'standard_key_snapshot', 'management_snapshot')),
    finality TEXT NOT NULL CHECK (finality IN ('authoritative', 'provisional', 'last_seen')),
    first_refreshed_at_utc TEXT NOT NULL DEFAULT (datetime('now')),
    last_refreshed_at_utc TEXT NOT NULL DEFAULT (datetime('now')),
    sample_count INTEGER NOT NULL DEFAULT 1,
    PRIMARY KEY (credential_profile_id, date_utc)
);

CREATE TABLE IF NOT EXISTS daily_activity_details (
    credential_profile_id INTEGER NOT NULL,
    date_utc TEXT NOT NULL,
    model TEXT NOT NULL,
    provider_name TEXT NOT NULL DEFAULT '',
    endpoint_id TEXT NOT NULL DEFAULT '',
    usage REAL NOT NULL DEFAULT 0,
    byok_usage REAL NOT NULL DEFAULT 0,
    prompt_tokens INTEGER NOT NULL DEFAULT 0,
    completion_tokens INTEGER NOT NULL DEFAULT 0,
    reasoning_tokens INTEGER NOT NULL DEFAULT 0,
    requests INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (credential_profile_id, date_utc, model, provider_name, endpoint_id)
);
