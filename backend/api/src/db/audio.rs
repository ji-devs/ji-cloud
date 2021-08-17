pub mod user {
    use futures::stream::BoxStream;
    use shared::domain::audio::{user::UserAudio, AudioId};
    use sqlx::PgPool;

    pub async fn create(db: &PgPool) -> sqlx::Result<AudioId> {
        let mut txn = db.begin().await?;

        let id: AudioId = sqlx::query!(
            r#"
insert into user_audio_library default values
returning id as "id: AudioId"
"#,
        )
        .fetch_one(db)
        .await?
        .id;

        sqlx::query!("insert into user_audio_upload (audio_id) values($1)", id.0)
            .execute(&mut *txn)
            .await?;

        txn.commit().await?;

        Ok(id)
    }

    pub async fn delete(db: &PgPool, image: AudioId) -> sqlx::Result<()> {
        sqlx::query!("delete from user_audio_library where id = $1", image.0)
            .execute(db)
            .await
            .map(drop)
    }

    pub async fn get(db: &PgPool, image: AudioId) -> sqlx::Result<Option<UserAudio>> {
        sqlx::query_as!(
            UserAudio,
            r#"select id as "id: AudioId" from user_audio_library where id = $1"#,
            image.0
        )
        .fetch_optional(db)
        .await
    }

    pub fn list(db: &PgPool) -> BoxStream<'_, sqlx::Result<UserAudio>> {
        sqlx::query_as!(
            UserAudio,
            r#"select id as "id: AudioId" from user_audio_library order by created_at desc"#,
        )
        .fetch(db)
    }
}
