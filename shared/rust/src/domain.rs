pub mod auth;
pub mod category;
pub mod image;
pub mod user;
pub mod meta;

fn deserialize_optional_field<'de, T, D>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    Ok(Some(serde::Deserialize::deserialize(deserializer)?))
}
