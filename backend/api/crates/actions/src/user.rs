use shared::{
    auth::{RegisterError, RegisterRequest},
    user::{NoSuchUserError, User},
};
use sqlx::postgres::PgPool;

impl From<UserQuery> for User {
    fn from(u: UserQuery) -> Self {
        Self {
            id: u.id,
            display_name: u.display_name,
            first_name: u.first_name,
            last_name: u.last_name,
            email: u.email,
            roles: u.roles.into_iter().map(|r| r.into()).collect(),
        }
    }
}

#[derive(sqlx::FromRow)]
pub struct UserQuery {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub roles: Vec<i32>,
    pub email: String,
    pub display_name: String,
}

pub async fn get_by_email(db: &PgPool, email_addr: &str) -> Option<User> {
    sqlx::query_as::<_, UserQuery>("SELECT * FROM users WHERE email = $1")
        .bind(email_addr)
        .fetch_optional(db)
        .await
        .expect("get by email shouldn't error")
        .map(|u: UserQuery| u.into())
}

pub async fn get_by_id(db: &PgPool, user_id: &str) -> Option<User> {
    sqlx::query_as::<_, UserQuery>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(db)
        .await
        .expect("get by id shouldn't error")
        .map(|u: UserQuery| u.into())
}

pub async fn get_profile(db: &PgPool, id: &str) -> Result<User, NoSuchUserError> {
    match get_by_id(&db, &id).await {
        None => Err(NoSuchUserError {}),
        Some(user) => Ok(user),
    }
}

pub async fn register(
    db: &PgPool,
    user_id: &str,
    req: &RegisterRequest,
) -> Result<(), RegisterError> {
    if get_by_id(&db, &user_id).await.is_some() {
        return Err(RegisterError::TakenId);
    }

    if req.display_name.is_empty() {
        return Err(RegisterError::EmptyDisplayname);
    }

    if req.first_name.is_empty() {
        return Err(RegisterError::EmptyFirstname);
    }

    if req.last_name.is_empty() {
        return Err(RegisterError::EmptyLastname);
    }

    if get_by_email(&db, &req.email).await.is_some() {
        return Err(RegisterError::TakenEmail);
    }

    sqlx::query(
        r#"
            INSERT INTO users 
                (id, display_name, first_name, last_name, email) 
            VALUES 
                ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(user_id)
    .bind(&req.display_name)
    .bind(&req.first_name)
    .bind(&req.last_name)
    .bind(&req.email)
    .execute(db)
    .await
    .expect("register: insert shouldn't fail");

    Ok(())
}
