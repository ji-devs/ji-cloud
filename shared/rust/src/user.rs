use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[cfg(feature = "backend")]
use sqlx::postgres::PgRow;
// note that this is unstable and will totally be split into many things later on
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(i8)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
pub enum UserScope {
    Admin = 1,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserProfile {
    pub firebase_id: String,
    pub scopes: Vec<UserScope>,
    pub email: String,
    pub display_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoSuchUserError {}

// HACK: we can't get `Vec<UserRole>` directly from the DB, so we have to work around it for now.
// see: https://github.com/launchbadge/sqlx/issues/298
#[cfg(feature = "backend")]
impl<'r> sqlx::FromRow<'r, PgRow> for UserProfile {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let DbUser {
            id,
            scopes,
            email,
            display_name,
            created_at,
            updated_at,
        } = DbUser::from_row(row)?;

        Ok(Self {
            firebase_id: id,
            scopes: scopes.into_iter().map(|(it,)| it).collect(),
            email,
            display_name,
            created_at,
            updated_at,
        })
    }
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
struct DbUser {
    pub id: String,
    pub scopes: Vec<(UserScope,)>,
    pub email: String,
    pub display_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
