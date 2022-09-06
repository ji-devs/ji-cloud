//! Serialization helpers

use serde::de::{DeserializeOwned, Visitor};
use std::{
    fmt::{self, Write},
    marker::PhantomData,
    str::FromStr,
};

use uuid::Uuid;

/// Hack to deserialize an Optional [`Option<T>`]
///
/// This is to differentiate between "missing" values and null values.
/// For example in json `{"v": null}` and `{}` are different things, in the first one, `v` is `null`, but in the second, v is `undefined`.
///
/// [`Option<T>`]: Option
pub fn deserialize_optional_field<'de, T, D>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    Ok(Some(serde::Deserialize::deserialize(deserializer)?))
}

/// Serializes a slice of hyphenated UUIDs into CSV format
///
/// ## Note:
/// * Algolia takes CSV format arrays: https://www.algolia.com/doc/rest-api/search/#arrays
pub fn csv_encode_uuids<T: Into<Uuid> + Copy, S>(
    uuids: &[T],
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    // todo: use a `Display` struct to use `collect_str`
    // but for now, pre-allocate the whole string.

    // a hyphenated uuid is 36 bytes long, we have `n` of those, then we also have `n - 1` 1 byte separators.
    let len = uuids.len() * 36 + uuids.len().saturating_sub(1);

    let mut out = String::with_capacity(len);
    let mut iter = uuids.iter().copied().map(<T as Into<Uuid>>::into);
    if let Some(item) = iter.next() {
        write!(&mut out, "{}", item.hyphenated())
            .expect("`String` call to `write!` shouldn't fail.");
    }

    for item in iter {
        write!(&mut out, ",{}", item.hyphenated())
            .expect("`String` call to `write!` shouldn't fail");
    }

    serializer.serialize_str(&out)
}

/// ASSUMING this is only going to be used for resources identified by index, which is
/// a non-negative integer which begins counting up from 0.
///
/// In most cases for this project, i16 is used instead of u16 because PostgreSQL does not have
/// unsigned integer types.
pub fn csv_encode_i16_indices<T: Into<i16> + Copy, S>(
    values: &[T],
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    // i16 has range [-32768, 32767].
    // We are making the assumption here that the values are >= 0.
    let len = values.len() * 5 + values.len().saturating_sub(1);

    let mut out = String::with_capacity(len);
    let mut iter = values.iter().copied().map(<T as Into<i16>>::into);
    if let Some(item) = iter.next() {
        write!(&mut out, "{}", item.to_string())
            .expect("`String` call to `write!` shouldn't fail.");
    }

    for item in iter {
        write!(&mut out, ",{}", item.to_string())
            .expect("`String` call to `write!` shouldn't fail");
    }

    serializer.serialize_str(&out)
}

/// Deserializes a slice of hyphenated UUIDs into CSV format
///
/// ## Note:
/// * Algolia takes CSV format arrays: https://www.algolia.com/doc/rest-api/search/#arrays
pub fn from_csv<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: DeserializeOwned,
{
    deserializer.deserialize_str(CSVVecVisitor::<T>::default())
}

/// Visits a string value of the form "v1,v2,v3" into a vector of bytes Vec<u8>
struct CSVVecVisitor<T: DeserializeOwned>(std::marker::PhantomData<T>);

impl<T: DeserializeOwned> Default for CSVVecVisitor<T> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<'de, T: DeserializeOwned> serde::de::Visitor<'de> for CSVVecVisitor<T> {
    type Value = Vec<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "a str")
    }

    fn visit_str<E>(self, s: &str) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(s.as_bytes())
            .into_deserialize()
            .next()
            .unwrap_or_else(|| Ok(Vec::new()))
            .map_err(|e| E::custom(format!("could not deserialize sequence value: {:?}", e)))
    }
}

// // I think this is commented out to avoid repeated writer re-allocations? the csv_encode_* functions
// // above preallocate. Not 100% certain if this is the reason, or if the performance impact is noticeable
// pub(super) fn vec_encode_csv<T: Serialize, S>(v: &Vec<T>, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: serde::Serializer,
// {
//     let mut writer = csv::WriterBuilder::new()
//         .has_headers(false)
//         .from_writer(vec![]);

//     writer.serialize(v).map_err(Error::custom)?;

//     // This error might not be triggerable.
//     let raw = writer.into_inner().map_err(Error::custom)?;

//     let s = std::str::from_utf8(&raw).map_err(Error::custom)?;

//     serializer.serialize_str(s)
// }

pub(super) struct FromStrVisitor<T>(pub PhantomData<T>);

impl<'de, TErr: std::fmt::Debug, T: FromStr<Err = TErr>> Visitor<'de> for FromStrVisitor<T> {
    type Value = T;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        FromStr::from_str(value)
            .map_err(|e| E::custom(format!("could not deserialize string: {:?}", e)))
    }
}
