/// TODO - use macros to keep it DRY, handle audio uploading in the same basic functions
use awsm_web::loaders::helpers::AbortController;
use shared::{
    api::{Method, PathParts},
    domain::{
        image::{user::*, *},
        CreateResponse,
    },
    media::MediaLibrary,
};
use thiserror::Error;
use utils::prelude::*;

use web_sys::File;

const STR_IMAGE_TOO_LARGE: &str = "Image is too large, limit is 30MB";

#[derive(Debug, Error)]
pub enum UploadError {
    #[error("Aborted")]
    Aborted,
    #[error("TooLarge")]
    TooLarge,
    #[error("awsm_web error")]
    Other(awsm_web::errors::Error),
}

impl UploadError {
    pub fn is_abort(&self) -> bool {
        match self {
            Self::Aborted => true,
            //Technically this won't ever be true, but does't hurt
            Self::Other(err) => err.is_abort(),
            _ => false,
        }
    }

    pub fn is_too_large(&self) -> bool {
        matches!(self, Self::TooLarge)
    }
}

/*
 * Need to handle cancellation at 3 levels:
 * 1. The API request to create a new image
 * 2. Upload to GCS
 * 3. Waiting for Firestore status update
 *
 * For the sake of convenience, errors are consolidated into the awsm_web Error type
 * This is fine since the API calls are EmptyError (which can map to Error::Empty)
 *
 * Cancellation only stops subsequent steps from happening.
 * Doesn't go back and delete previous steps
 */

pub async fn upload_image(
    id: ImageId,
    lib: MediaLibrary,
    file: &File,
    abort_controller: Option<&AbortController>,
) -> Result<(), UploadError> {
    let endpoint = match lib {
        MediaLibrary::Global => ImageUploadPath(id.clone()).get_filled(),
        MediaLibrary::User => UserImageUploadPath(id.clone()).get_filled(),
        _ => panic!("Cannot upload images other than to global or user library!"),
    };

    let resp = api_upload_file_with_auth_abortable::<(), ()>(
        &endpoint,
        Method::Put,
        file,
        None,
        abort_controller,
    )
    .await
    .map_err(|aborted| {
        if aborted {
            UploadError::Aborted
        } else {
            UploadError::Other(awsm_web::errors::Error::Empty)
        }
    });

    if let Ok((_, status)) = resp {
        side_effect_status_code(status).await;
    }

    resp.and_then(|(resp, status)| {
        if status == 413 {
            let _ = web_sys::window()
                .unwrap_ji()
                .alert_with_message(STR_IMAGE_TOO_LARGE);
            Err(UploadError::TooLarge)
        } else {
            resp.map_err(UploadError::Other)
        }
    })
}

pub async fn upload_user_image(
    size: ImageSize,
    file: &File,
    abort_controller: Option<&AbortController>,
) -> Result<ImageId, UploadError> {
    let req = UserImageCreateRequest { size };
    let resp = api_upload_file_with_auth_abortable::<CreateResponse<ImageId>, _>(
        &UserImageCreatePath().get_filled(),
        Method::Post,
        file,
        Some(req),
        abort_controller,
    )
    .await
    .map_err(|aborted| {
        if aborted {
            UploadError::Aborted
        } else {
            UploadError::Other(awsm_web::errors::Error::Empty)
        }
    });

    if let Ok((_, status)) = resp {
        side_effect_status_code(status).await;
    }

    let resp = resp.and_then(|(resp, status)| {
        if status == 413 {
            let _ = web_sys::window()
                .unwrap_ji()
                .alert_with_message(STR_IMAGE_TOO_LARGE);
            Err(UploadError::TooLarge)
        } else {
            resp.map_err(UploadError::Other)
        }
    })?;

    Ok(resp.id)
}
