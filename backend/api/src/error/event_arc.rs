use paperclip::actix::api_v2_errors;

use super::{ise, BasicError};
use http::StatusCode;

#[api_v2_errors(code = 400, code = 401, code = 404, code = 501)]
#[derive(Debug)]
pub enum EventArc {
    InternalServerError(anyhow::Error),
    // UNSTABLE!
    // todo: potentially cover other error responses from google API
    InvalidEventSource,
    InvalidEventResource,
    Disabled,
    NotProcessed,
}

impl<T: Into<anyhow::Error>> From<T> for EventArc {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

// Status codes for EventArc which respond to the Google Pub/Sub pushes need to be one of
// { 102, 200, 201, 202, 204 } to indicated that the subscription has been received.
// See https://cloud.google.com/pubsub/docs/push.
impl Into<actix_web::Error> for EventArc {
    fn into(self) -> actix_web::Error {
        match self {
            Self::Disabled => super::ServiceKind::GoogleCloudEventArc.into(),
            Self::InvalidEventSource => BasicError::with_message(
                StatusCode::NO_CONTENT,
                "Non-supported event type received".to_owned(),
            )
            .into(),
            Self::InvalidEventResource => BasicError::with_message(
                StatusCode::NO_CONTENT,
                "Non-supported resource received".to_owned(),
            )
            .into(),
            Self::NotProcessed => {
                BasicError::with_message(StatusCode::ACCEPTED, "Media was not processed".to_owned())
                    .into()
            }
            Self::InternalServerError(e) => ise(e),
        }
    }
}
