//! Common configuration for the ji-cloud Rust backend servers.

#![deny(missing_docs)]

/// Environment helpers.
pub(crate) mod env;

/// Interaction with google.
pub mod google;

/// Common http configuration.
pub mod http;

/// Keeps track of settings.
pub mod settings;
