use actix_web::web::ServiceConfig;
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
    use actix_web::{
        web::{Data, Json, Path},
        HttpResponse,
    };
    use futures::TryStreamExt;
    use shared::{
        api::{endpoints, ApiEndpoint},
        domain::{
            audio::{
                user::{
                    UserAudio, UserAudioListResponse, UserAudioResponse, UserAudioUploadResponse,
                },
                AudioId,
            },
            CreateResponse,
        },
        media::{FileKind, MediaLibrary},
    };
    use sqlx::PgPool;

    use crate::{
        db, error,
        extractor::{RequestOrigin, TokenUser},
        service::{s3, storage, GcpAccessKeyStore, ServiceData},
    };

    /// Create a audio file in the user's audio library.
    pub(super) async fn create(
        db: Data<PgPool>,
        _claims: TokenUser,
    ) -> Result<HttpResponse, error::NotFound> {
        let id = db::audio::user::create(db.as_ref()).await?;
        Ok(HttpResponse::Created().json(CreateResponse { id }))
    }

    /// upload a audio file to the user's audio library.
    pub(super) async fn upload(
        db: Data<PgPool>,
        gcp_key_store: ServiceData<GcpAccessKeyStore>,
        gcs: ServiceData<storage::Client>,
        _claims: TokenUser,
        id: Path<AudioId>,
        origin: RequestOrigin,
        req: Json<<endpoints::audio::user::Upload as ApiEndpoint>::Req>,
    ) -> Result<Json<<endpoints::audio::user::Upload as ApiEndpoint>::Res>, error::Upload> {
        let id = id.into_inner();

        let mut txn = db.begin().await?;

        let exists = sqlx::query!(
        r#"select exists(select 1 from user_audio_upload where audio_id = $1 for no key update) as "exists!""#,
        id.0
    )
            .fetch_one(&mut txn)
            .await?.exists;

        if !exists {
            return Err(error::Upload::ResourceNotFound);
        }

        let upload_content_length = req.into_inner().file_size;

        if let Some(file_limit) = gcs.file_size_limit(&FileKind::AudioMp3) {
            if file_limit < upload_content_length {
                return Err(error::Upload::FileTooLarge);
            }
        }

        let access_token = gcp_key_store.fetch_token().await?.to_owned();

        let resp = gcs
            .get_url_for_resumable_upload_for_processing(
                &access_token,
                upload_content_length,
                MediaLibrary::User,
                id.0,
                FileKind::AudioMp3,
                origin,
            )
            .await?;

        sqlx::query!(
        "update user_audio_upload set uploaded_at = now(), processing_result = null where audio_id = $1",
        id.0
    )
        .execute(&mut txn)
        .await?;

        txn.commit().await?;

        Ok(Json(UserAudioUploadResponse { session_uri: resp }))
    }

    /// Delete a audio file from the user's audio library.
    pub(super) async fn delete(
        db: Data<PgPool>,
        _claims: TokenUser,
        req: Path<AudioId>,
        s3: ServiceData<s3::Client>,
    ) -> Result<HttpResponse, error::Delete> {
        let audio = req.into_inner();
        db::audio::user::delete(&db, audio)
            .await
            .map_err(super::check_conflict_delete)?;

        s3.delete_media(MediaLibrary::User, FileKind::AudioMp3, audio.0)
            .await;

        Ok(HttpResponse::NoContent().finish())
    }

    /// Get a audio file from the user's audio library.
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

pub fn configure(cfg: &mut ServiceConfig) {
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
