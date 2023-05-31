use std::env;

pub(crate) mod keys;

/// Reads a required env value as a `String`, throwing an error if it is missing.
pub fn req_env(key: &str) -> anyhow::Result<String> {
    env::var(key).map_err(|_| anyhow::anyhow!("Missing required env var `{}`", key))
}

/// Reads a boolean value from the .env file. Maps any of {"true", "1", "y"} to true, and
/// all others to false.
pub fn env_bool(key: &str) -> bool {
    env::var(key).map_or(false, |it| ["true", "1", "y"].contains(&it.as_ref()))
}
