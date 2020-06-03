use serde::{Deserialize, Serialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum UserRole {
    Admin = 1,
}

impl From<i32> for UserRole {
    fn from(x:i32) -> Self {
        if x == UserRole::Admin as i32 {
            UserRole::Admin
        } else {
            panic!("Couldn't get user role from i32!");
        }
    }
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
