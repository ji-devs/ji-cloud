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

pub mod auth;
pub mod category;
pub mod image;
pub mod meta;
mod ser;
pub mod user;

use ser::{csv_encode_uuids, deserialize_optional_field, from_csv};
