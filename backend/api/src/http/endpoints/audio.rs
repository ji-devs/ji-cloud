use actix_web::web::ServiceConfig;
use shared::api::{endpoints::audio, ApiEndpoint, PathParts};
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
        web::{Data, Json, Path, Payload},
        HttpResponse,
    };
    use futures::TryStreamExt;
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

    use crate::{
        db, error,
        extractor::TokenUser,
        service::{s3, upload as upload_service, ServiceData},
    };

    /// Create a audio file in the user's audio library.
    pub(super) async fn create(
        db: Data<PgPool>,
        s3: ServiceData<s3::Client>,
        _claims: TokenUser,
        payload: Payload,
    ) -> Result<HttpResponse, error::Upload> {
        let file = super::super::read_limited_payload(payload, FileKind::AudioMp3).await?;
        let id = db::audio::user::create(db.as_ref()).await?;

        let mut txn = db.begin().await?;
        upload_service::process_user_audio_bytes(&mut txn, &s3, id.0, file).await?;
        txn.commit().await?;

        Ok(HttpResponse::Created().json(CreateResponse { id }))
    }

    /// upload a audio file to the user's audio library.
    pub(super) async fn upload(
        db: Data<PgPool>,
        s3: ServiceData<s3::Client>,
        _claims: TokenUser,
        id: Path<AudioId>,
        payload: Payload,
    ) -> Result<HttpResponse, error::Upload> {
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

        let file = super::super::read_limited_payload(payload, FileKind::AudioMp3).await?;

        upload_service::process_user_audio_bytes(&mut txn, &s3, id.0, file).await?;

        txn.commit().await?;

        Ok(HttpResponse::Ok().finish())
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
        <audio::user::Create as ApiEndpoint>::Path::PATH,
        audio::user::Create::METHOD.route().to(self::user::create),
    )
    .route(
        <audio::user::Upload as ApiEndpoint>::Path::PATH,
        audio::user::Upload::METHOD.route().to(self::user::upload),
    )
    .route(
        <audio::user::Delete as ApiEndpoint>::Path::PATH,
        audio::user::Delete::METHOD.route().to(self::user::delete),
    )
    .route(
        <audio::user::Get as ApiEndpoint>::Path::PATH,
        audio::user::Get::METHOD.route().to(self::user::get),
    )
    .route(
        <audio::user::List as ApiEndpoint>::Path::PATH,
        audio::user::List::METHOD.route().to(self::user::list),
    );
}
