use paperclip::actix::api_v2_errors;

use crate::google_oauth;

use super::BasicError;

#[api_v2_errors(code = 400, code = 401, code = 409, code = 403, code = 500, code = 501)]
#[derive(Debug)]
pub enum OAuth {
    InternalServerError(anyhow::Error),
    Google(GoogleOAuth),
    Conflict,
}

impl<T: Into<anyhow::Error>> From<T> for OAuth {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl From<GoogleOAuth> for OAuth {
    fn from(google: GoogleOAuth) -> Self {
        Self::Google(google)
    }
}

impl Into<actix_web::Error> for OAuth {
    fn into(self) -> actix_web::Error {
        match self {
            Self::InternalServerError(err) => super::ise(err),
            Self::Google(it) => it.into(),
            Self::Conflict => BasicError::with_message(
                http::StatusCode::CONFLICT,
                "User with same email exists, but they haven't enabled oauth".to_owned(),
            )
            .into(),
        }
    }
}

#[api_v2_errors(code = 400, code = 401, code = 403, code = 500, code = 501)]
#[derive(Debug)]
pub enum GoogleOAuth {
    InternalServerError(anyhow::Error),
    // todo: this entire format is unstable, so it's okay to iterate on this a bunch
    Disabled,
    InvalidCode,
    RedirectUriMismatch,
    UnverifiedEmail,
}

impl From<google_oauth::TokenErrorResponse> for GoogleOAuth {
    fn from(err: google_oauth::TokenErrorResponse) -> Self {
        match err {
            google_oauth::TokenErrorResponse::Unknown(map) => {
                anyhow::anyhow!("Unknown {:?}", map).into()
            }
            google_oauth::TokenErrorResponse::Known(it) => Self::from(it),
        }
    }
}

impl From<google_oauth::TokenErrorKind> for GoogleOAuth {
    fn from(err: google_oauth::TokenErrorKind) -> Self {
        match err {
            google_oauth::TokenErrorKind::InvalidClient { error_description } => {
                anyhow::anyhow!(error_description)
                    .context("invalid client")
                    .into()
            }

            google_oauth::TokenErrorKind::InvalidGrant => Self::InvalidCode,
            google_oauth::TokenErrorKind::RedirectUriMismatch => Self::RedirectUriMismatch,
        }
    }
}

impl<T: Into<anyhow::Error>> From<T> for GoogleOAuth {
    fn from(e: T) -> Self {
        Self::InternalServerError(e.into())
    }
}

impl Into<actix_web::Error> for GoogleOAuth {
    fn into(self) -> actix_web::Error {
        match self {
            Self::InternalServerError(e) => super::ise(e.context("google oauth error")),

            Self::Disabled => super::ServiceKind::GoogleOAuth.into(),

            GoogleOAuth::InvalidCode => {
                BasicError::with_message(http::StatusCode::UNAUTHORIZED, "Invalid Code".to_owned())
                    .into()
            }

            GoogleOAuth::RedirectUriMismatch => BasicError::with_message(
                http::StatusCode::UNAUTHORIZED,
                "Redirect URI Mismatch".to_owned(),
            )
            .into(),

            GoogleOAuth::UnverifiedEmail => BasicError::with_message(
                http::StatusCode::UNAUTHORIZED,
                "Email isn't verified".to_owned(),
            )
            .into(),
        }
    }
}
