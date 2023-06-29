//! Types that travel over the wire.

macro_rules! into_i16_index {
    ( $( $t:ty ),+ $(,)? ) => {
        $(
            impl From<$t> for i16 {
                fn from(t: $t) -> Self {
                    t.0
                }
            }

            /// Needed to cast i16 into i64 range for algolia indexing
            impl From<$t> for i64 {
                fn from(t: $t) -> Self {
                    t.0 as i64
                }
            }
        )+
    };
}

/// Helper macro to generate a Newtype that wraps a [uuid::Uuid], derives relevant macros
/// and sets it up to be stored in the database.
///
/// Example:
///
/// ```
/// wrap_uuid! {
///   /// Represents a My ID
///   #[serde(rename_all = "camelCase")]
///   pub struct MyId
/// }
/// ```
macro_rules! wrap_uuid {
    (
        $(#[$outer:meta])*
        $vis:vis struct $t:ident
    ) => {
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize, PathPart, Hash)]
        $(#[$outer])*
        #[cfg_attr(feature = "backend", derive(sqlx::Type))]
        #[cfg_attr(feature = "backend", sqlx(transparent))]
        $vis struct $t(pub uuid::Uuid);

        impl From<$t> for uuid::Uuid {
            fn from(t: $t) -> Self {
                t.0
            }
        }

        impl std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl std::str::FromStr for $t {
            type Err = uuid::Error;

            #[inline]
            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Ok(Self(uuid::Uuid::from_str(value)?))
            }
        }

        impl $t {
            /// Creates a wrapped UUID from a 128 bit value
            #[inline]
            #[must_use]
            $vis const fn from_u128(v: u128) -> Self {
                Self(uuid::Uuid::from_u128(v))
            }
        }
    }
}

pub mod additional_resource;
pub mod admin;
pub mod animation;
pub mod asset;
pub mod audio;
pub mod billing;
pub mod category;
pub mod circle;
pub mod course;
pub mod image;
pub mod jig;
pub mod locale;
pub mod media;
pub mod meta;
pub mod module;
pub mod pdf;
pub mod playlist;
pub mod resource;
pub mod search;
pub mod ser;
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
    pub use super::user::CreateProfileRequest as RegisterRequest;
}

use chrono::Utc;
use ser::{csv_encode_i16_indices, csv_encode_uuids, deserialize_optional_field, from_csv};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

/// Serialize/Deserialize wrapper for Base64 encoded content.
#[derive(Debug)]
pub struct Base64<T>(pub T);

impl<T: Display> serde::Serialize for Base64<T> {
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
        Ok(Self(deserializer.deserialize_str(ser::FromStrVisitor(
            std::marker::PhantomData,
        ))?))
    }
}
/// Response for successfuly creating a Resource.
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct CreateResponse<T: Into<Uuid>> {
    /// The newly created resource's ID.
    pub id: T,
}

/// Represents when to publish an image.
#[derive(Copy, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Debug)]
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
                // todo: error instead of panicking
                Utc::now() + chrono::Duration::from_std(d).expect("Really really big duration?")
            }
        }
    }
}

/// Clearer representation of an optional nullable field.
///
/// Requires `#[serde(default, skip_serializing_if = "Update::is_keep")]` to be applied to
/// fields which use this type.
#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum UpdateNullable<T> {
    /// Use the current value stored in the database. Equivalent of `undefined` in JS.
    Keep,
    /// Set the value to `null` or the equivalent.
    Unset,
    /// Use the given value.
    Change(T),
}

impl<T> Default for UpdateNullable<T> {
    fn default() -> Self {
        Self::Keep
    }
}

impl<'de, T> Deserialize<'de> for UpdateNullable<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        #[serde(untagged)]
        enum UpdateMap<T> {
            Unset,
            Change(T),
        }

        let mapping = UpdateMap::deserialize(deserializer)?;

        let update = match mapping {
            UpdateMap::Unset => Self::Unset,
            UpdateMap::Change(val) => Self::Change(val),
        };

        Ok(update)
    }
}

impl<T> UpdateNullable<T> {
    /// Whether this is the `Keep` variant
    pub const fn is_keep(&self) -> bool {
        matches!(self, Self::Keep)
    }

    /// Whether this is the `Keep` variant
    pub const fn is_unset(&self) -> bool {
        matches!(self, Self::Unset)
    }

    /// Whether this is the `Change` variant
    pub const fn is_change(&self) -> bool {
        matches!(self, Self::Change(_))
    }

    /// Similar to `Option<Option<T>>::flatten()`, this converts the variant into an `Option<T>`.
    ///
    /// Useful for coalesce updates.
    pub fn into_option(self) -> Option<T> {
        match self {
            Self::Keep | Self::Unset => None,
            Self::Change(v) => Some(v),
        }
    }
}

/// Clearer representation of an optional non-nullable field.
///
/// Requires `#[serde(default, skip_serializing_if = "Update::is_keep")]` to be applied to
/// fields which use this type.
#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum UpdateNonNullable<T> {
    /// Use the current value stored in the database. Equivalent of `undefined` in JS.
    Keep,
    /// Use the given value.
    Change(T),
}

impl<T> Default for UpdateNonNullable<T> {
    fn default() -> Self {
        Self::Keep
    }
}

impl<'de, T> Deserialize<'de> for UpdateNonNullable<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self::Change(T::deserialize(deserializer)?))
    }
}

impl<T> UpdateNonNullable<T> {
    /// Whether this is the `Keep` variant
    pub const fn is_keep(&self) -> bool {
        matches!(self, Self::Keep)
    }

    /// Whether this is the `Change` variant
    pub const fn is_change(&self) -> bool {
        matches!(self, Self::Change(_))
    }

    /// Similar to `Option<Option<T>>::flatten()`, this converts the variant into an `Option<T>`.
    ///
    /// Useful for coalesce updates.
    pub fn into_option(self) -> Option<T> {
        match self {
            Self::Keep => None,
            Self::Change(v) => Some(v),
        }
    }
}

impl<T> From<Option<T>> for UpdateNonNullable<T> {
    fn from(value: Option<T>) -> Self {
        value.map(UpdateNonNullable::Change).unwrap_or_default()
    }
}

/// New-type representing the current page of a list of items
#[derive(Copy, Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct Page(usize);

impl From<usize> for Page {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<Page> for usize {
    fn from(value: Page) -> Self {
        value.0
    }
}

#[cfg(feature = "backend")]
impl From<Page> for i64 {
    fn from(value: Page) -> Self {
        value.0 as i64
    }
}

impl Display for Page {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Page {
    /// Get an instance of the next page
    pub fn next_page(self) -> Self {
        Self(self.0.saturating_add(1))
    }

    /// Get an instance of the previous page
    pub fn prev_page(self) -> Self {
        Self(self.0.saturating_sub(1))
    }
}

const DEFAULT_PAGE_LIMIT: usize = 20;

/// New-type representing the item limit for a page of items
#[derive(Serialize, Deserialize, Copy, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PageLimit(usize);

impl Default for PageLimit {
    fn default() -> Self {
        Self(DEFAULT_PAGE_LIMIT)
    }
}

impl From<usize> for PageLimit {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<PageLimit> for usize {
    fn from(value: PageLimit) -> Self {
        value.0
    }
}

#[cfg(feature = "backend")]
impl From<PageLimit> for i64 {
    fn from(value: PageLimit) -> Self {
        value.0 as i64
    }
}

impl PageLimit {
    /// Calculate the offset of items given the current page
    #[cfg(feature = "backend")]
    pub fn offset(&self, page: Page) -> i64 {
        (self.0 * page.0) as i64
    }
}

/// New-type representing the total count of items
#[derive(Serialize, Deserialize, Copy, Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ItemCount(usize);

impl From<usize> for ItemCount {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<ItemCount> for usize {
    fn from(value: ItemCount) -> Self {
        value.0
    }
}

#[cfg(feature = "backend")]
impl From<ItemCount> for i64 {
    fn from(value: ItemCount) -> Self {
        value.0 as i64
    }
}

impl ItemCount {
    /// Calculate the page count for a list of items
    pub fn paged(&self, limit: PageLimit) -> Self {
        // let pages = (total_count / (page_limit as u64)
        //     + (total_count % (page_limit as u64) != 0) as u64) as u32;
        let page_count = self.0 / limit.0 + (self.0 % limit.0 != 0) as usize;
        page_count.into()
    }
}

// use actix_web::{
//     http::{header::IntoHeaderPair, StatusCode},
//     HttpRequest, HttpResponse,
// };

// FIXME
// #[cfg(feature = "backend")]
// impl actix_web::Responder for CreateResponse<T: Into<Uuid>> {
//     type Future = futures::ready::Ready<HttpResponse>;
//
//     fn respond_to(self, _: &HttpRequest) -> Self::Future {
//         ready(Ok(HttpResponse::build(StatusCode::NO_CONTENT)
//             .content_type("application/json")
//             .finish()))
//     }
// }
