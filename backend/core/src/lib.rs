//! Common configuration for the ji-cloud Rust backend servers.

#![deny(missing_docs)]

/// Environment helpers.
pub mod env;

/// Interaction with google.
pub mod google;

/// Common http configuration.
pub mod http;

/// Keeps track of settings.
pub mod settings;

/// sentry integration
pub mod sentry;
