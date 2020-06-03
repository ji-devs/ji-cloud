use crate::db::Db;
use crate::schema::users::dsl::*;
use super::model::UserQuery;
use ji_cloud_shared::user::User;
use diesel::{QueryResult, NotFound, OptionalExtension};
use diesel::prelude::*;


pub fn get_by_email(db:&Db, email_addr:&str) -> Option<User> {
    users
        .filter(email.eq(email_addr))
        .first(db)
        .optional()
        .unwrap()
        .map(|u:UserQuery| u.into())
}


pub fn get_by_id(db:&Db, user_id:&str) -> Option<User> {
    users
        .find(user_id)
        .first(db)
        .optional()
        .unwrap()
        .map(|u:UserQuery| u.into())
}
