//! URL signing and verification utilities for share links.

use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;
use url::{form_urlencoded::Serializer, Url};

type HmacSha256 = Hmac<Sha256>;

const SIGNATURE_PARAM: &str = "s";
const DUMMY_BASE: &str = "http://dummy";

/// Signs a URL path and query string, returning the full signed URL with signature appended.
///
/// The signature covers the entire path and query string (canonicalized).
/// The signature is appended as `&s=<base64>` (or `?s=<base64>` if no query).
pub fn sign_url(base_url: &str, path_and_query: &str, secret: &[u8; 32]) -> String {
    let canonical = canonicalize_path(path_and_query);
    let signature = compute_signature(&canonical, secret);

    let full_url = format!("{}{}", base_url, path_and_query);
    let mut parsed = Url::parse(&full_url).expect("valid URL");

    parsed
        .query_pairs_mut()
        .append_pair(SIGNATURE_PARAM, &signature);

    parsed.to_string()
}

/// Verifies a signed URL. Returns the URL without the signature if valid.
///
/// Returns `None` if the signature is missing or invalid.
pub fn verify_signed_url(path_and_query: &str, secret: &[u8; 32]) -> Option<String> {
    let (url_without_sig, provided_sig) = extract_signature(path_and_query)?;

    let canonical = canonicalize_path(&url_without_sig);
    let expected_sig = compute_signature(&canonical, secret);

    if constant_time_eq(provided_sig.as_bytes(), expected_sig.as_bytes()) {
        Some(url_without_sig)
    } else {
        None
    }
}

/// Canonicalizes a path for signing by sorting query parameters alphabetically.
fn canonicalize_path(path_and_query: &str) -> String {
    let full_url = format!("{}{}", DUMMY_BASE, path_and_query);
    let parsed = match Url::parse(&full_url) {
        Ok(u) => u,
        Err(_) => return path_and_query.to_string(),
    };

    let path = parsed.path();
    let mut params: Vec<(String, String)> = parsed
        .query_pairs()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    if params.is_empty() {
        return path.to_string();
    }

    params.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

    let mut query_str = Serializer::new(String::new());
    query_str.extend_pairs(params.iter().map(|(k, v)| (k.as_str(), v.as_str())));
    let query_str = query_str.finish();

    format!("{}?{}", path, query_str)
}

/// Computes HMAC-SHA256 signature and returns base64url-encoded result.
fn compute_signature(data: &str, secret: &[u8; 32]) -> String {
    let mut mac = HmacSha256::new_from_slice(secret).expect("HMAC can take key of any size");
    mac.update(data.as_bytes());
    let result = mac.finalize();
    base64::encode_config(result.into_bytes(), base64::URL_SAFE_NO_PAD)
}

/// Extracts the signature from a URL and returns (url_without_sig, signature).
fn extract_signature(path_and_query: &str) -> Option<(String, String)> {
    let full_url = format!("{}{}", DUMMY_BASE, path_and_query);
    let parsed = Url::parse(&full_url).ok()?;

    let mut signature_value = None;
    let mut other_params = Vec::new();

    for (key, value) in parsed.query_pairs() {
        if key == SIGNATURE_PARAM {
            signature_value = Some(value.to_string());
        } else {
            other_params.push((key.to_string(), value.to_string()));
        }
    }

    let signature = signature_value?;

    let url_without_sig = if other_params.is_empty() {
        parsed.path().to_string()
    } else {
        let mut query_str = Serializer::new(String::new());
        query_str.extend_pairs(other_params.iter().map(|(k, v)| (k.as_str(), v.as_str())));
        format!("{}?{}", parsed.path(), query_str.finish())
    };

    Some((url_without_sig, signature))
}

/// Constant-time comparison to prevent timing attacks.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASE: &str = "https://example.com";

    #[test]
    fn test_sign_and_verify() {
        let secret: [u8; 32] = [0x42; 32];
        let path = "/asset/play/jig/abc123?draft_or_live=live&is_student=false";

        let signed = sign_url(BASE, path, &secret);
        assert!(signed.contains("&s="));
        assert!(signed.starts_with(BASE));

        // Extract path_and_query for verification
        let path_and_query = signed.strip_prefix(BASE).unwrap();
        let verified = verify_signed_url(path_and_query, &secret);
        assert_eq!(verified, Some(path.to_string()));
    }

    #[test]
    fn test_tampered_url_fails() {
        let secret: [u8; 32] = [0x42; 32];
        let path = "/asset/play/jig/abc123?draft_or_live=live&is_student=false";

        let signed = sign_url(BASE, path, &secret);
        let tampered = signed.replace("is_student=false", "is_student=true");

        let path_and_query = tampered.strip_prefix(BASE).unwrap();
        let verified = verify_signed_url(path_and_query, &secret);
        assert_eq!(verified, None);
    }

    #[test]
    fn test_wrong_secret_fails() {
        let secret1: [u8; 32] = [0x42; 32];
        let secret2: [u8; 32] = [0x43; 32];
        let path = "/asset/play/jig/abc123?draft_or_live=live";

        let signed = sign_url(BASE, path, &secret1);
        let path_and_query = signed.strip_prefix(BASE).unwrap();
        let verified = verify_signed_url(path_and_query, &secret2);
        assert_eq!(verified, None);
    }

    #[test]
    fn test_canonicalization_order_independent() {
        let secret: [u8; 32] = [0x42; 32];
        let path1 = "/asset/play/jig/abc?a=1&b=2";
        let path2 = "/asset/play/jig/abc?b=2&a=1";

        let signed1 = sign_url(BASE, path1, &secret);
        let signed2 = sign_url(BASE, path2, &secret);

        // Extract signatures - they should be the same due to canonicalization
        let sig1 = signed1.split("s=").nth(1).unwrap();
        let sig2 = signed2.split("s=").nth(1).unwrap();
        assert_eq!(sig1, sig2);
    }

    #[test]
    fn test_url_without_query() {
        let secret: [u8; 32] = [0x42; 32];
        let path = "/asset/play/jig/abc123";

        let signed = sign_url(BASE, path, &secret);
        assert!(signed.contains("?s="));

        let path_and_query = signed.strip_prefix(BASE).unwrap();
        let verified = verify_signed_url(path_and_query, &secret);
        assert_eq!(verified, Some(path.to_string()));
    }
}
