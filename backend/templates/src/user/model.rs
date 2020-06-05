use diesel::prelude::*;
use ji_cloud_shared::user::User;

impl From<UserQuery> for User {
    fn from(u:UserQuery) -> Self {
        Self {
            id: u.id,
            display_name: u.display_name,
            first_name: u.first_name,
            last_name: u.last_name,
            email: u.email,
            roles: u.roles.into_iter().map(|r| r.into()).collect()
        }
    }
}

#[derive(Queryable)]
pub struct UserQuery {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub roles: Vec<i32>,
    pub email: String,
    pub display_name: String,
}
