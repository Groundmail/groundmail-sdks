use std::time::{Duration, SystemTime, UNIX_EPOCH};

use hmac::{Hmac, Mac};
use sha2::Sha256;

/// How far a webhook's timestamp may be from now.
pub const DEFAULT_TOLERANCE: Duration = Duration::from_secs(300);

/// Checks a `Groundmail-Signature` header against the raw request body, within five minutes.
///
/// Pass the body exactly as it arrived, before parsing it. `secret` is the endpoint's secret.
pub fn verify_webhook_signature(secret: &str, payload: &[u8], header: &str) -> bool {
    verify_webhook_signature_with_tolerance(secret, payload, header, DEFAULT_TOLERANCE)
}

/// [`verify_webhook_signature`] with your own tolerance.
pub fn verify_webhook_signature_with_tolerance(
    secret: &str,
    payload: &[u8],
    header: &str,
    tolerance: Duration,
) -> bool {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    verify_at(secret, payload, header, tolerance, now)
}

fn verify_at(secret: &str, payload: &[u8], header: &str, tolerance: Duration, now: u64) -> bool {
    let mut timestamp = None;
    let mut signature = None;
    for part in header.split(',') {
        match part.trim().split_once('=') {
            Some(("t", value)) => timestamp = value.parse::<u64>().ok(),
            Some(("v1", value)) => signature = hex::decode(value).ok(),
            _ => {}
        }
    }
    let (Some(timestamp), Some(signature)) = (timestamp, signature) else {
        return false;
    };
    if now.abs_diff(timestamp) > tolerance.as_secs() {
        return false;
    }
    let Ok(mut mac) = Hmac::<Sha256>::new_from_slice(secret.as_bytes()) else {
        return false;
    };
    mac.update(timestamp.to_string().as_bytes());
    mac.update(b".");
    mac.update(payload);
    mac.verify_slice(&signature).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    const AT: u64 = 1_793_000_000;
    const BODY: &[u8] = br#"{"type":"email.delivered"}"#;
    // Signed by the Node SDK's scheme, so the two stay in step.
    const SIGNATURE: &str = "88afe6a191a6136657e18e545c40515598a05d1aab05a670c306308382edc969";

    fn header() -> String {
        format!("t={AT},v1={SIGNATURE}")
    }

    #[test]
    fn accepts_a_valid_signature() {
        assert!(verify_at(
            "whsec_test",
            BODY,
            &header(),
            DEFAULT_TOLERANCE,
            AT + 10
        ));
    }

    #[test]
    fn rejects_a_wrong_secret_or_changed_body() {
        assert!(!verify_at(
            "whsec_other",
            BODY,
            &header(),
            DEFAULT_TOLERANCE,
            AT
        ));
        assert!(!verify_at(
            "whsec_test",
            b"{}",
            &header(),
            DEFAULT_TOLERANCE,
            AT
        ));
    }

    #[test]
    fn rejects_an_old_or_future_timestamp() {
        assert!(!verify_at(
            "whsec_test",
            BODY,
            &header(),
            DEFAULT_TOLERANCE,
            AT + 301
        ));
        assert!(!verify_at(
            "whsec_test",
            BODY,
            &header(),
            DEFAULT_TOLERANCE,
            AT - 301
        ));
    }

    #[test]
    fn rejects_a_malformed_header() {
        for bad in ["", "t=abc,v1=00", "t=1793000000", "v1=00", "nonsense"] {
            assert!(!verify_at("whsec_test", BODY, bad, DEFAULT_TOLERANCE, AT));
        }
    }
}
