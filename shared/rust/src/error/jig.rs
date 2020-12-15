//! Errors for JIG routes.

use crate::domain::meta::MetaKind;

// workarounds for backwards compat, if you see this comment please remove references to this and remove this module

/// Error occurred while creating a Resource.
pub type CreateError = super::CreateError<CreateErrorExt>;

/// Error occurred while updating a Resource.
pub type UpdateError = super::UpdateError<UpdateErrorExt>;

/// Extension Error for [`CreateError`]
#[non_exhaustive]
#[cfg_attr(
    feature = "backend",
    paperclip::actix::api_v2_errors(
        code = 420,
        description = "Unprocessable Entity: Metadata associated with this operation could not be found"
    )
)]
#[derive(serde::Serialize, serde::Deserialize)]
pub enum CreateErrorExt {
    /// Metadata associated with this operation could not be found.
    NonExistantMetadata {
        /// The (Optional) id of the item.
        id: Option<uuid::Uuid>,
        /// The item's kind.
        kind: MetaKind,
    },
}

/// Extension Error for [`UpdateError`]
pub type UpdateErrorExt = CreateErrorExt;

#[cfg(feature = "backend")]
impl From<CreateErrorExt> for actix_web::Error {
    fn from(e: CreateErrorExt) -> actix_web::Error {
        match e {
            e @ CreateErrorExt::NonExistantMetadata { .. } => {
                actix_web::HttpResponse::UnprocessableEntity()
                    .json(e)
                    .into()
            }
        }
    }
}
