use crate::env::req_env;

/// Initializes sentry with the given dsn and remote_target
pub fn init(
    dsn: Option<&str>,
    remote_target: shared::config::RemoteTarget,
) -> anyhow::Result<sentry::ClientInitGuard> {
    // Sample rate defaults to 0.4 if not set as an environment variable
    let traces_sample_rate =
        req_env("SENTRY_SAMPLE_RATE").map_or(Ok(0.4), |value| value.parse::<f32>())?;

    let dsn = dsn.unwrap_or("");
    let options = sentry::ClientOptions {
        dsn: sentry::IntoDsn::into_dsn(dsn)?,
        environment: Some(std::borrow::Cow::Borrowed(remote_target.as_str())),
        traces_sample_rate,
        ..Default::default()
    };

    let options = sentry::apply_defaults(options);

    Ok(sentry::init(options))
}
