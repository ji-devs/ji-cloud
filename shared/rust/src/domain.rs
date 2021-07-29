//! Types that travel over the wire.

macro_rules! into_uuid {
    ( $( $t:ty ),+ $(,)? ) => {
        $(
            impl From<$t> for uuid::Uuid {
                fn from(t: $t) -> Self {
                    t.0
                }
            }
        )+
    };
}

pub mod admin;
pub mod animation;
pub mod audio;
pub mod category;
pub mod image;
pub mod jig;
pub mod locale;
pub mod media;
pub mod meta;
pub mod search;
mod ser;
pub mod session;
pub mod user;

#[deprecated]
/// auth types (deprecated)
pub mod auth {

    #[deprecated]
    pub use super::session::AUTH_COOKIE_NAME;

    #[deprecated]
    pub use super::session::CSRF_HEADER_NAME;

    #[deprecated]
    pub use super::user::PutProfileRequest as RegisterRequest;
}

use chrono::Utc;
#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use ser::{csv_encode_uuids, deserialize_optional_field, from_csv};
use uuid::Uuid;

/// Serialize/Deserialize wrapper for Base64 encoded content.
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
#[cfg_attr(feature = "backend", openapi(empty))]
#[derive(Debug)]
pub struct Base64<T>(pub T);

impl<T: std::fmt::Display> serde::Serialize for Base64<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&base64::encode(&self.0.to_string()))
    }
}

impl<'de, E: std::fmt::Debug, T: std::str::FromStr<Err = E>> serde::Deserialize<'de> for Base64<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self(deserializer.deserialize_str(ser::FromStrVisiter(
            std::marker::PhantomData,
        ))?))
    }
}
/// Response for successfuly creating a Resource.
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub struct CreateResponse<T: Into<Uuid>> {
    /// The newly created resource's ID.
    pub id: T,
}

/// Represents when to publish an image.
#[derive(Copy, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(Apiv2Schema))]
pub enum Publish {
    /// Publish the image *at* the given time.
    At(chrono::DateTime<Utc>),
    /// Publish the image *in* the given amount of time from now.
    In(std::time::Duration),
}

impl Publish {
    /// creates an instance of `Self` that will publish "right now"
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn now() -> Self {
        // Duration::new is const unstable
        Self::In(std::time::Duration::new(0, 0))
    }
}

impl From<Publish> for chrono::DateTime<Utc> {
    fn from(publish: Publish) -> Self {
        match publish {
            Publish::At(t) => t,
            Publish::In(d) => {
                // todo: error instead of panicing
                Utc::now() + chrono::Duration::from_std(d).expect("Really really big duration?")
            }
        }
    }
}
