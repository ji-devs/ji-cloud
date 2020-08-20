pub mod env;
pub mod google;
pub mod http;

#[cfg(any(feature = "local", feature = "sandbox", feature = "release",))]
pub mod settings;

#[cfg(any(feature = "local", feature = "sandbox", feature = "release"))]
use config::RemoteTarget;

#[cfg(not(any(feature = "local", feature = "sandbox", feature = "release")))]
compile_error!("At least one of the `local`, `sandbox` or `release` features must be enabled.");

#[cfg(any(
    all(feature = "local", feature = "sandbox"),
    all(feature = "local", feature = "release"),
    all(feature = "sandbox", feature = "release"),
))]
compile_error!("Only one of `local`, `sandbox` or `release` features can be enabled.");

#[cfg(feature = "local")]
pub(crate) const REMOTE_TARGET: RemoteTarget = RemoteTarget::Local;

#[cfg(feature = "sandbox")]
pub(crate) const REMOTE_TARGET: RemoteTarget = RemoteTarget::Sandbox;

#[cfg(feature = "release")]
pub(crate) const REMOTE_TARGET: RemoteTarget = RemoteTarget::Release;
