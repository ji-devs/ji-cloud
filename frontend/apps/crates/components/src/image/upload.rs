/// TODO - use macros to keep it DRY, handle audio uploading in the same basic functions
use crate::firebase;
use awsm_web::loaders::helpers::AbortController;
use shared::{
    api::{endpoints, ApiEndpoint},
    domain::image::{user::*, *},
    error::*,
    media::MediaLibrary,
};
use utils::prelude::*;

use web_sys::File;

const STR_IMAGE_TOO_LARGE: &'static str = "Image is too large, limit is 30MB";

#[derive(Debug)]
pub enum UploadError {
    Aborted,
    TooLarge,
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
        match self {
            Self::TooLarge => true,
            _ => false,
        }
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
    let session_uri = {
        match lib {
            MediaLibrary::Global => {
                let req = ImageUploadRequest {
                    file_size: file.size() as usize,
                };

                let path = endpoints::image::Upload::PATH.replace("{id}", &id.0.to_string());

                let resp = api_with_auth_status_abortable::<ImageUploadResponse, EmptyError, _>(
                    &path,
                    endpoints::image::Upload::METHOD,
                    abort_controller,
                    Some(req),
                )
                .await
                .map_err(|aborted| {
                    if aborted {
                        UploadError::Aborted
                    } else {
                        UploadError::Other(awsm_web::errors::Error::Empty)
                    }
                })
                .and_then(|(resp, status)| {
                    if status == 413 {
                        let _ = web_sys::window()
                            .unwrap_ji()
                            .alert_with_message(STR_IMAGE_TOO_LARGE);
                        Err(UploadError::TooLarge)
                    } else {
                        side_effect_status_code(status);
                        resp.map_err(|_| UploadError::Other(awsm_web::errors::Error::Empty))
                    }
                })?;

                let ImageUploadResponse { session_uri } = resp;
                session_uri
            }

            MediaLibrary::User => {
                let req = UserImageUploadRequest {
                    file_size: file.size() as usize,
                };

                let path = endpoints::image::user::Upload::PATH.replace("{id}", &id.0.to_string());

                let resp =
                    api_with_auth_status_abortable::<UserImageUploadResponse, EmptyError, _>(
                        &path,
                        endpoints::image::user::Upload::METHOD,
                        abort_controller,
                        Some(req),
                    )
                    .await
                    .map_err(|aborted| {
                        if aborted {
                            UploadError::Aborted
                        } else {
                            UploadError::Other(awsm_web::errors::Error::Empty)
                        }
                    })
                    .and_then(|(resp, status)| {
                        if status == 413 {
                            let _ = web_sys::window()
                                .unwrap_ji()
                                .alert_with_message(STR_IMAGE_TOO_LARGE);
                            Err(UploadError::TooLarge)
                        } else {
                            side_effect_status_code(status);
                            resp.map_err(|_| UploadError::Other(awsm_web::errors::Error::Empty))
                        }
                    })?;

                let UserImageUploadResponse { session_uri } = resp;
                session_uri
            }

            _ => panic!("Cannot upload images other than to global or user library!"),
        }
    };

    //upload to GCS
    upload_file_gcs(&session_uri, &file, abort_controller)
        .await
        .map_err(|err| {
            if err.is_abort() {
                UploadError::Aborted
            } else {
                UploadError::Other(err)
            }
        })?;

    log::info!(
        "{} uploaded, waiting for processing to start...",
        id.0.to_string()
    );

    if firebase::wait_for_upload_ready(&id.0, lib, abort_controller).await {
        Ok(())
    } else {
        match abort_controller {
            Some(a) => {
                if a.signal().aborted() {
                    Err(UploadError::Aborted)
                } else {
                    Err(UploadError::Other(awsm_web::errors::Error::Empty))
                }
            }
            None => Err(UploadError::Other(awsm_web::errors::Error::Empty)),
        }
    }
}
