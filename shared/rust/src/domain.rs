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

pub mod animation;
pub mod audio;
pub mod auth;
pub mod category;
pub mod image;
pub mod jig;
pub mod meta;
mod ser;
pub mod user;

use chrono::Utc;
#[cfg(feature = "backend")]
use paperclip::actix::Apiv2Schema;
use ser::{csv_encode_uuids, deserialize_optional_field, from_csv};
use uuid::Uuid;

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
