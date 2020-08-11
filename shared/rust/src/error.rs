#[cfg(feature = "backend")]
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

#[cfg(feature = "backend")]
fn anyhow_to_ise(e: anyhow::Error) -> actix_web::Error {
    let mut resp = actix_web::HttpResponse::InternalServerError();
    resp.extensions_mut().insert(e);
    resp.into()
}
