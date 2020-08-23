use std::env;

pub fn req_env(key: &str) -> anyhow::Result<String> {
    env::var(key).map_err(|_| anyhow::anyhow!("Missing required env var `{}`", key))
}

pub fn env_bool(key: &str) -> bool {
    env::var(key).map_or(false, |it| ["true", "1", "y"].contains(&it.as_ref()))
}
