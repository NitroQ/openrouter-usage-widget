use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Authentication failed: {0}")]
    AuthError(String),
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Storage error: {0}")]
    StorageError(String),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("OpenRouter error: {0}")]
    OpenRouterError(String),
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display_auth() {
        let err = AppError::AuthError("bad key".into());
        assert_eq!(err.to_string(), "Authentication failed: bad key");
    }

    #[test]
    fn error_display_network() {
        let err = AppError::NetworkError("timeout".into());
        assert_eq!(err.to_string(), "Network error: timeout");
    }

    #[test]
    fn error_display_not_found() {
        let err = AppError::NotFound("resource".into());
        assert_eq!(err.to_string(), "Not found: resource");
    }

    #[test]
    fn error_display_storage() {
        let err = AppError::StorageError("disk full".into());
        assert_eq!(err.to_string(), "Storage error: disk full");
    }

    #[test]
    fn error_display_invalid_input() {
        let err = AppError::InvalidInput("bad mode".into());
        assert_eq!(err.to_string(), "Invalid input: bad mode");
    }

    #[test]
    fn error_display_openrouter() {
        let err = AppError::OpenRouterError("rate limited".into());
        assert_eq!(err.to_string(), "OpenRouter error: rate limited");
    }

    #[test]
    fn error_serialize() {
        let err = AppError::AuthError("test".into());
        let json = serde_json::to_string(&err).unwrap();
        assert_eq!(json, "\"Authentication failed: test\"");
    }

    #[test]
    fn error_is_debug() {
        let err = AppError::NetworkError("x".into());
        let debug = format!("{err:?}");
        assert!(debug.contains("NetworkError"));
    }
}
