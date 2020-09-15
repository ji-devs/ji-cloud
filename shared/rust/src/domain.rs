//! Types that travel over the wire.

pub mod auth;
pub mod category;
pub mod image;
pub mod meta;
pub mod user;

/// Hack to deserialize an Optional [`Option<T>`]
///
/// This is to differentiate between "missing" values and null values.
/// For example in json `{"v": null}` and `{}` are different things, in the first one, `v` is `null`, but in the second, v is `undefined`.
///
/// [`Option<T>`]: https://doc.rust-lang.org/stable/std/option/enum.Option.html
fn deserialize_optional_field<'de, T, D>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    Ok(Some(serde::Deserialize::deserialize(deserializer)?))
}
