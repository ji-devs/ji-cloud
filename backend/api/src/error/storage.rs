use paperclip::actix::api_v2_errors;

use crate::google::storage;

use super::{ise, BasicError};
use actix_web_httpauth::headers::authorization::Basic;
use http::StatusCode;

#[api_v2_errors(code = 400, code = 401, code = 404, code = 501)]
#[derive(Debug)]
pub enum Storage {
    InternalServerError(anyhow::Error),
    // UNSTABLE!
    // todo: potentially cover other error responses from google API
    InvalidGrant,
    Disabled,
    FileTooLarge,
}

impl From<storage::UploadUrlErrorResponse> for Storage {
    fn from(err: storage::UploadUrlErrorResponse) -> Self {
        match err {
            storage::UploadUrlErrorResponse::Unknown(map) => {
                anyhow::anyhow!("Unknown {:?}", map).into()
            }
        }
    }
}

impl<T: Into<anyhow::Error>> From<T> for Storage {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for Storage {
    fn into(self) -> actix_web::Error {
        match self {
            Self::Disabled => super::ServiceKind::GoogleCloudStorage.into(),
            Self::FileTooLarge => super::Upload::FileTooLarge.into(),
            Self::InvalidGrant => BasicError::with_message(
                StatusCode::UNAUTHORIZED,
                "Unauthorized request to Google Cloud Storage. Does the requestor have sufficient permissions?".to_owned()
            ).into(),
            Self::InternalServerError(e) => ise(e),
        }
    }
}
