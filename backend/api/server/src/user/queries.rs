use super::model::UserQuery;
use ji_cloud_shared::user::User;
use sqlx::postgres::PgPool;

pub fn get_by_email(db:&PgPool, email_addr:&str) -> Option<User> {
    log::error!("TODO - get db record via email");
    None
    /*
    users
        .filter(email.eq(email_addr))
        .first(db)
        .optional()
        .unwrap()
        .map(|u:UserQuery| u.into())
        */
}


pub fn get_by_id(db:&PgPool, user_id:&str) -> Option<User> {
    log::error!("TODO - get db record via id");
    None
    /*
    users
        .find(user_id)
        .first(db)
        .optional()
        .unwrap()
        .map(|u:UserQuery| u.into())
        */
}
