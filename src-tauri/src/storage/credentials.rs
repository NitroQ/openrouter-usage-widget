use sha2::{Digest, Sha256};

const SERVICE_NAME: &str = "openrouter-widget";
const ACCOUNT_NAME: &str = "openrouter_api_key";

pub fn compute_fingerprint(api_key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(api_key.as_bytes());
    let result = hasher.finalize();
    hex::encode(&result[..16])
}

pub fn save_credential(api_key: &str) -> Result<(), String> {
    let entry = keyring::Entry::new(SERVICE_NAME, ACCOUNT_NAME)
        .map_err(|e| format!("Failed to create credential entry: {e}"))?;

    entry
        .set_password(api_key)
        .map_err(|e| format!("Failed to save credential: {e}"))?;

    Ok(())
}

pub fn load_credential() -> Option<String> {
    let entry = keyring::Entry::new(SERVICE_NAME, ACCOUNT_NAME).ok()?;
    entry.get_password().ok()
}

pub fn delete_credential() -> Result<(), String> {
    let entry = keyring::Entry::new(SERVICE_NAME, ACCOUNT_NAME)
        .map_err(|e| format!("Failed to create credential entry: {e}"))?;

    entry
        .delete_credential()
        .map_err(|e| format!("Failed to delete credential: {e}"))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fingerprint_deterministic() {
        let key = "sk-or-v1-test123456";
        let fp1 = compute_fingerprint(key);
        let fp2 = compute_fingerprint(key);
        assert_eq!(fp1, fp2);
    }

    #[test]
    fn fingerprint_different_for_different_keys() {
        let fp1 = compute_fingerprint("sk-or-v1-key1");
        let fp2 = compute_fingerprint("sk-or-v1-key2");
        assert_ne!(fp1, fp2);
    }

    #[test]
    fn fingerprint_length() {
        let fp = compute_fingerprint("test");
        // SHA-256 truncated to 16 bytes = 32 hex chars
        assert_eq!(fp.len(), 32);
    }

    #[test]
    fn fingerprint_only_hex() {
        let fp = compute_fingerprint("test-key-for-hex-check");
        assert!(fp.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn fingerprint_empty_key() {
        let fp = compute_fingerprint("");
        assert_eq!(fp.len(), 32);
    }

    #[test]
    fn fingerprint_does_not_contain_original() {
        let key = "my-secret-api-key-12345";
        let fp = compute_fingerprint(key);
        assert!(!fp.contains(key));
    }
}
