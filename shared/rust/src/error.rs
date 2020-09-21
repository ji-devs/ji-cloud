//! Home of the error types.

/// Generates a `From` impl to convert from `Into<anyhow::Error>` to an enum
/// with a `InternalServerError(anyhow::Error)` variant.
macro_rules! from_anyhow {
    ( $( $t:ty ),+ $(,)? ) => {
        $(
            impl<T: Into<anyhow::Error>> From<T> for $t {
                fn from(e: T) -> Self {
                    Self::InternalServerError(e.into())
                }
            }
        )+
    };
}

pub mod auth;
pub mod category;
pub mod image;

/// User errors.
pub mod user {

    /// The user does not exist.
    #[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
    pub struct NoSuchUserError {}
}

/// Converts from an [`anyhow::Error`] to a http `InternalServerError`.
///
/// [`anyhow::Error`]: ../../anyhow/struct.Error.html
#[cfg(feature = "backend")]
fn anyhow_to_ise(e: anyhow::Error) -> actix_web::Error {
    let mut resp = actix_web::HttpResponse::InternalServerError();
    // put the contents of the error into an extension to avoid the client seeing what the error is, and so that the log picks it up.
    resp.extensions_mut().insert(e);
    resp.into()
}

/// Represents an error from the backend.
pub struct InternalServerError(pub anyhow::Error);

impl<T: Into<anyhow::Error>> From<T> for InternalServerError {
    fn from(e: T) -> Self {
        InternalServerError(e.into())
    }
}

#[cfg(feature = "backend")]
impl From<InternalServerError> for actix_web::Error {
    fn from(e: InternalServerError) -> actix_web::Error {
        anyhow_to_ise(e.0)
    }
}
