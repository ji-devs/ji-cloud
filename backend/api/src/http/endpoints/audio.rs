use paperclip::actix::web::ServiceConfig;
use shared::{
    api::{endpoints::audio, ApiEndpoint},
    error::DeleteError,
};
use sqlx::postgres::PgDatabaseError;

fn check_conflict_delete(err: sqlx::Error) -> DeleteError {
    match err {
        sqlx::Error::Database(e) if e.downcast_ref::<PgDatabaseError>().constraint().is_some() => {
            DeleteError::Conflict
        }
        _ => DeleteError::InternalServerError(err.into()),
    }
}

pub mod user {
    use crate::{db, extractor::WrapAuthClaimsNoDb, s3::S3Client};
    use actix_web::{http, HttpResponse};
    use futures::TryStreamExt;
    use paperclip::actix::{
        api_v2_operation,
        web::{Bytes, Data, Json, Path},
        CreatedJson,
    };
    use shared::{
        api::{endpoints, ApiEndpoint},
        domain::{
            audio::{
                user::{GetResponse, ListResponse, UserAudio},
                AudioId,
            },
            CreateResponse,
        },
        error::{audio::UploadError, GetError},
        media::MediaLibraryKind,
        media::MediaVariant,
    };
    use sqlx::PgPool;

    #[api_v2_operation]
    pub(super) async fn create(
        db: Data<PgPool>,
        _claims: WrapAuthClaimsNoDb,
    ) -> Result<
        CreatedJson<<endpoints::audio::user::Create as ApiEndpoint>::Res>,
        <endpoints::audio::user::Create as ApiEndpoint>::Err,
    > {
        let id = db::audio::user::create(db.as_ref()).await?;
        Ok(CreatedJson(CreateResponse { id }))
    }

    #[api_v2_operation]
    pub(super) async fn upload(
        db: Data<PgPool>,
        s3: Data<S3Client>,
        _claims: WrapAuthClaimsNoDb,
        Path(id): Path<AudioId>,
        bytes: Bytes,
    ) -> Result<HttpResponse, <endpoints::audio::user::Upload as ApiEndpoint>::Err> {
        if !db::audio::user::exists(db.as_ref(), id).await? {
            return Err(shared::error::audio::UploadError::NotFound);
        }

        // todo: use the duration
        let _duration = {
            let bytes = bytes.clone();
            tokio::task::spawn_blocking(move || {
                mp3_metadata::read_from_slice(&bytes).map_err(|_it| UploadError::InvalidAudio)
            })
            .await
            .unwrap()?
        };

        s3.upload_audio(MediaLibraryKind::User, id, bytes.to_vec())
            .await?;

        Ok(HttpResponse::NoContent().into())
    }

    #[api_v2_operation]
    pub(super) async fn delete(
        db: Data<PgPool>,
        _claims: WrapAuthClaimsNoDb,
        req: Path<AudioId>,
        s3: Data<S3Client>,
    ) -> Result<HttpResponse, <endpoints::audio::user::Delete as ApiEndpoint>::Err> {
        let audio = req.into_inner();
        db::audio::user::delete(&db, audio)
            .await
            .map_err(super::check_conflict_delete)?;

        s3.delete_audio(MediaLibraryKind::Global, MediaVariant::Original, audio)
            .await;

        Ok(HttpResponse::new(http::StatusCode::NO_CONTENT))
    }

    #[api_v2_operation]
    pub(super) async fn get(
        db: Data<PgPool>,
        _claims: WrapAuthClaimsNoDb,
        req: Path<AudioId>,
    ) -> Result<
        Json<<endpoints::audio::user::Get as ApiEndpoint>::Res>,
        <endpoints::audio::user::Get as ApiEndpoint>::Err,
    > {
        let metadata = db::audio::user::get(&db, req.into_inner())
            .await?
            .ok_or(GetError::NotFound)?;

        Ok(Json(GetResponse { metadata }))
    }

    #[api_v2_operation]
    pub(super) async fn list(
        db: Data<PgPool>,
        _claims: WrapAuthClaimsNoDb,
    ) -> Result<
        Json<<endpoints::audio::user::List as ApiEndpoint>::Res>,
        <endpoints::audio::user::List as ApiEndpoint>::Err,
    > {
        let audio_files: Vec<_> = db::audio::user::list(db.as_ref())
            .err_into::<GetError>()
            .and_then(|metadata: UserAudio| async { Ok(GetResponse { metadata }) })
            .try_collect()
            .await?;

        Ok(Json(ListResponse { audio_files }))
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
