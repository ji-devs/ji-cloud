/// Initializes sentry with the given dsn and remote_target
pub fn init(
    dsn: &str,
    remote_target: config::RemoteTarget,
) -> anyhow::Result<sentry::ClientInitGuard> {
    let options = sentry::ClientOptions {
        dsn: Some(
            sentry::IntoDsn::into_dsn(dsn)?
                .ok_or_else(|| anyhow::anyhow!("failed to initialize sentry"))?,
        ),
        environment: Some(std::borrow::Cow::Borrowed(remote_target.as_str())),
        ..Default::default()
    };

    let options = sentry::apply_defaults(options);

    Ok(sentry::init(options))
}
