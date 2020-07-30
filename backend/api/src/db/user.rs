use shared::{auth::RegisterRequest, user::User};

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

pub async fn by_email(db: &sqlx::PgPool, email_addr: &str) -> sqlx::Result<Option<User>> {
    let user = sqlx::query_as::<_, UserQuery>("SELECT * FROM users WHERE email = $1")
        .bind(email_addr)
        .fetch_optional(db)
        .await?
        .map(User::from);

    Ok(user)
}

pub async fn by_id(db: &sqlx::PgPool, user_id: &str) -> sqlx::Result<Option<User>> {
    let user = sqlx::query_as::<_, UserQuery>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(db)
        .await?
        .map(|u: UserQuery| u.into());

    Ok(user)
}

pub async fn register(db: &sqlx::PgPool, user_id: &str, req: &RegisterRequest) -> sqlx::Result<()> {
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
    .map(drop)
}
