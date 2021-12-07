pub mod user {
    use futures::stream::BoxStream;
    use shared::domain::pdf::{user::UserPdf, PdfId};
    use sqlx::PgPool;
    use uuid::Uuid;

    pub async fn create(db: &PgPool, user_id: Uuid) -> sqlx::Result<PdfId> {
        let mut txn = db.begin().await?;

        let id: PdfId = sqlx::query!(
            r#"
insert into user_pdf_library(user_id)
values($1)
returning id as "id: PdfId"
        "#,
            user_id
        )
        .fetch_one(db)
        .await?
        .id;

        sqlx::query!("insert into user_pdf_upload (pdf_id) values($1)", id.0)
            .execute(&mut *txn)
            .await?;

        txn.commit().await?;

        Ok(id)
    }

    pub async fn delete(db: &PgPool, image: PdfId) -> sqlx::Result<()> {
        sqlx::query!("delete from user_pdf_library where id = $1", image.0)
            .execute(db)
            .await
            .map(drop)
    }

    pub async fn get(db: &PgPool, image: PdfId) -> sqlx::Result<Option<UserPdf>> {
        sqlx::query_as!(
            UserPdf,
            r#"select id as "id: PdfId" from user_pdf_library where id = $1"#,
            image.0
        )
        .fetch_optional(db)
        .await
    }

    pub fn list(db: &PgPool) -> BoxStream<'_, sqlx::Result<UserPdf>> {
        sqlx::query_as!(
            UserPdf,
            r#"select id as "id: PdfId" from user_pdf_library order by created_at desc"#,
        )
        .fetch(db)
    }
}
