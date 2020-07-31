use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
#[cfg(feature = "backend")]
use sqlx::postgres::PgRow;

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(i8)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
pub enum UserRole {
    Admin = 1,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub roles: Vec<UserRole>,
    pub email: String,
    pub display_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoSuchUserError {}

// HACK: we can't get `Vec<UserRole>` directly from the DB, so we have to work around it for now.
// see: https://github.com/launchbadge/sqlx/issues/298
#[cfg(feature = "backend")]
impl<'r> sqlx::FromRow<'r, PgRow> for User {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let DbUser {
            id,
            first_name,
            last_name,
            roles,
            email,
            display_name,
        } = DbUser::from_row(row)?;

        Ok(Self {
            id,
            first_name,
            last_name,
            roles: roles.into_iter().map(|(it,)| it).collect(),
            email,
            display_name,
        })
    }
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
struct DbUser {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub roles: Vec<(UserRole,)>,
    pub email: String,
    pub display_name: String,
}
