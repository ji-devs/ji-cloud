use paperclip::actix::web::ServiceConfig;
use shared::api::{endpoints::audio, ApiEndpoint};
use sqlx::postgres::PgDatabaseError;

use crate::error;

fn check_conflict_delete(err: sqlx::Error) -> error::Delete {
    match err {
        sqlx::Error::Database(e) if e.downcast_ref::<PgDatabaseError>().constraint().is_some() => {
            error::Delete::Conflict
        }
        _ => error::Delete::InternalServerError(err.into()),
    }
}

pub mod user {
    use crate::{db, error, extractor::TokenUser, s3, service::ServiceData};
    use futures::TryStreamExt;
    use paperclip::actix::{
        api_v2_operation,
        web::{Bytes, Data, Json, Path},
        CreatedJson, NoContent,
    };
    use shared::{
        api::{endpoints, ApiEndpoint},
        domain::{
            audio::{
                user::{UserAudio, UserAudioListResponse, UserAudioResponse},
                AudioId,
            },
            CreateResponse,
        },
        media::{FileKind, MediaLibrary},
    };
    use sqlx::PgPool;

    /// Create a audio file in the user's audio library.
    #[api_v2_operation]
    pub(super) async fn create(
        db: Data<PgPool>,
        _claims: TokenUser,
    ) -> Result<CreatedJson<<endpoints::audio::user::Create as ApiEndpoint>::Res>, error::NotFound>
    {
        let id = db::audio::user::create(db.as_ref()).await?;
        Ok(CreatedJson(CreateResponse { id }))
    }

    /// upload a audio file to the user's audio library.
    #[api_v2_operation]
    pub(super) async fn upload(
        db: Data<PgPool>,
        s3: ServiceData<s3::Client>,
        _claims: TokenUser,
        Path(id): Path<AudioId>,
        bytes: Bytes,
    ) -> Result<NoContent, error::Upload> {
        let mut txn = db.begin().await?;

        sqlx::query!(
            r#"select 1 as discard from user_audio_library where id = $1 for update"#,
            id.0
        )
        .fetch_optional(&mut txn)
        .await?
        .ok_or(error::Upload::ResourceNotFound)?;

        // todo: use the duration
        let _duration = {
            let bytes = bytes.clone();
            tokio::task::spawn_blocking(move || {
                mp3_metadata::read_from_slice(&bytes).map_err(|_it| error::Upload::InvalidMedia)
            })
            .await
            .unwrap()?
        };

        s3.upload_media(bytes.to_vec(), MediaLibrary::User, id.0, FileKind::AudioMp3)
            .await?;

        sqlx::query!(
            "update user_audio_library set uploaded_at = now() where id = $1",
            id.0
        )
        .execute(&mut txn)
        .await?;

        txn.commit().await?;

        Ok(NoContent)
    }

    /// Delete a audio file from the user's audio library.
    #[api_v2_operation]
    pub(super) async fn delete(
        db: Data<PgPool>,
        _claims: TokenUser,
        req: Path<AudioId>,
        s3: ServiceData<s3::Client>,
    ) -> Result<NoContent, error::Delete> {
        let audio = req.into_inner();
        db::audio::user::delete(&db, audio)
            .await
            .map_err(super::check_conflict_delete)?;

        s3.delete_media(MediaLibrary::User, FileKind::AudioMp3, audio.0)
            .await;

        Ok(NoContent)
    }

    /// Get a audio file from the user's audio library.
    #[api_v2_operation]
    pub(super) async fn get(
        db: Data<PgPool>,
        _claims: TokenUser,
        req: Path<AudioId>,
    ) -> Result<Json<<endpoints::audio::user::Get as ApiEndpoint>::Res>, error::NotFound> {
        let metadata = db::audio::user::get(&db, req.into_inner())
            .await?
            .ok_or(error::NotFound::ResourceNotFound)?;

        Ok(Json(UserAudioResponse { metadata }))
    }

    /// List audio files from the user's audio library.
    #[api_v2_operation]
    pub(super) async fn list(
        db: Data<PgPool>,
        _claims: TokenUser,
    ) -> Result<Json<<endpoints::audio::user::List as ApiEndpoint>::Res>, error::Server> {
        let audio_files: Vec<_> = db::audio::user::list(db.as_ref())
            .err_into::<error::Server>()
            .and_then(|metadata: UserAudio| async { Ok(UserAudioResponse { metadata }) })
            .try_collect()
            .await?;

        Ok(Json(UserAudioListResponse { audio_files }))
    }
}

pub fn configure(cfg: &mut ServiceConfig<'_>) {
    cfg.route(
        audio::user::Create::PATH,
        audio::user::Create::METHOD.route().to(self::user::create),
    )
    .route(
        audio::user::Upload::PATH,
        audio::user::Upload::METHOD.route().to(self::user::upload),
    )
    .route(
        audio::user::Delete::PATH,
        audio::user::Delete::METHOD.route().to(self::user::delete),
    )
    .route(
        audio::user::Get::PATH,
        audio::user::Get::METHOD.route().to(self::user::get),
    )
    .route(
        audio::user::List::PATH,
        audio::user::List::METHOD.route().to(self::user::list),
    );
}
