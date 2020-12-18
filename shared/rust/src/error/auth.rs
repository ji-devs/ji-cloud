//! Errors for Auth routes.

use serde::{Deserialize, Serialize};

/// An error occured during registration
#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterError {
    /// Which kind of error occurred.
    pub kind: RegisterErrorKind,
}

#[non_exhaustive]
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
/// Represents the kinds of errors that can occur during registration.
pub enum RegisterErrorKind {
    /// No username was provided.
    EmptyDisplayName,

    /// Another user with the provided email already exists.
    TakenEmail,

    /// Another user with the provided firebase-id already exists.
    TakenId,

    /// Another user with the provided username already exists.
    TakenUsername,
}
