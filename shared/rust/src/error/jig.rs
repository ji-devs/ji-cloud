//! Errors for JIG routes.

// workarounds for backwards compat, if you see this comment please remove references to this and remove this module

/// Error occurred while creating a Resource.
pub type CreateError = super::CreateError;

/// Error occurred while updating a Resource.
pub type UpdateError = super::UpdateError;
