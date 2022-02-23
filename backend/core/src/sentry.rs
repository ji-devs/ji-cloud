/// Initializes sentry with the given dsn and remote_target
pub fn init(
    dsn: Option<&str>,
    remote_target: shared::config::RemoteTarget,
) -> anyhow::Result<sentry::ClientInitGuard> {
    let dsn = dsn.unwrap_or("");
    let options = sentry::ClientOptions {
        dsn: sentry::IntoDsn::into_dsn(dsn)?,
        environment: Some(std::borrow::Cow::Borrowed(remote_target.as_str())),
        traces_sample_rate: 0.2,
        ..Default::default()
    };

    let options = sentry::apply_defaults(options);

    Ok(sentry::init(options))
}
