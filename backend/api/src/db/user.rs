use shared::{auth::RegisterRequest, user::User};

pub async fn by_email(db: &sqlx::PgPool, email_addr: &str) -> sqlx::Result<Option<User>> {
    sqlx::query_as("SELECT * FROM users WHERE email = $1")
        .bind(email_addr)
        .fetch_optional(db)
        .await
}

pub async fn by_id(db: &sqlx::PgPool, user_id: &str) -> sqlx::Result<Option<User>> {
    sqlx::query_as("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(db)
        .await
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
