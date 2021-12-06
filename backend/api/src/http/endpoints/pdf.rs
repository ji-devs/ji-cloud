use actix_web::web::ServiceConfig;
use shared::api::{endpoints::pdf, ApiEndpoint};
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
            pdf::{
                user::{UserPdf, UserPdfListResponse, UserPdfResponse, UserPdfUploadResponse},
                PdfId,
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

    /// Create a pdf file in the user's pdf library.
    pub(super) async fn create(
        db: Data<PgPool>,
        _claims: TokenUser,
    ) -> Result<HttpResponse, error::NotFound> {
        let id = db::pdf::user::create(db.as_ref()).await?;
        Ok(HttpResponse::Created().json(CreateResponse { id }))
    }

    /// upload a pdf file to the user's pdf library.
    pub(super) async fn upload(
        db: Data<PgPool>,
        gcp_key_store: ServiceData<GcpAccessKeyStore>,
        gcs: ServiceData<storage::Client>,
        _claims: TokenUser,
        id: Path<PdfId>,
        origin: RequestOrigin,
        req: Json<<endpoints::pdf::user::Upload as ApiEndpoint>::Req>,
    ) -> Result<Json<<endpoints::pdf::user::Upload as ApiEndpoint>::Res>, error::Upload> {
        let id = id.into_inner();

        let mut txn = db.begin().await?;

        let exists = sqlx::query!(
        r#"select exists(select 1 from user_pdf_upload where pdf_id = $1 for no key update) as "exists!""#,
        id.0
    )
            .fetch_one(&mut txn)
            .await?.exists;

        if !exists {
            return Err(error::Upload::ResourceNotFound);
        }

        let upload_content_length = req.into_inner().file_size;

        if let Some(file_limit) = gcs.file_size_limit(&FileKind::DocumentPdf) {
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
                FileKind::DocumentPdf,
                origin,
            )
            .await?;

        sqlx::query!(
        "update user_pdf_upload set uploaded_at = now(), processing_result = null where pdf_id = $1",
        id.0
    )
        .execute(&mut txn)
        .await?;

        txn.commit().await?;

        Ok(Json(UserPdfUploadResponse { session_uri: resp }))
    }

    /// Delete a pdf file from the user's pdf library.
    pub(super) async fn delete(
        db: Data<PgPool>,
        _claims: TokenUser,
        req: Path<PdfId>,
        s3: ServiceData<s3::Client>,
    ) -> Result<HttpResponse, error::Delete> {
        let pdf = req.into_inner();
        db::pdf::user::delete(&db, pdf)
            .await
            .map_err(super::check_conflict_delete)?;

        s3.delete_media(MediaLibrary::User, FileKind::DocumentPdf, pdf.0)
            .await;

        Ok(HttpResponse::NoContent().finish())
    }

    /// Get a pdf file from the user's pdf library.
    pub(super) async fn get(
        db: Data<PgPool>,
        _claims: TokenUser,
        req: Path<PdfId>,
    ) -> Result<Json<<endpoints::pdf::user::Get as ApiEndpoint>::Res>, error::NotFound> {
        let metadata = db::pdf::user::get(&db, req.into_inner())
            .await?
            .ok_or(error::NotFound::ResourceNotFound)?;

        Ok(Json(UserPdfResponse { metadata }))
    }

    /// List pdf files from the user's pdf library.
    pub(super) async fn list(
        db: Data<PgPool>,
        _claims: TokenUser,
    ) -> Result<Json<<endpoints::pdf::user::List as ApiEndpoint>::Res>, error::Server> {
        let pdf_files: Vec<_> = db::pdf::user::list(db.as_ref())
            .err_into::<error::Server>()
            .and_then(|metadata: UserPdf| async { Ok(UserPdfResponse { metadata }) })
            .try_collect()
            .await?;

        Ok(Json(UserPdfListResponse { pdf_files }))
    }
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.route(
        pdf::user::Create::PATH,
        pdf::user::Create::METHOD.route().to(self::user::create),
    )
    .route(
        pdf::user::Upload::PATH,
        pdf::user::Upload::METHOD.route().to(self::user::upload),
    )
    .route(
        pdf::user::Delete::PATH,
        pdf::user::Delete::METHOD.route().to(self::user::delete),
    )
    .route(
        pdf::user::Get::PATH,
        pdf::user::Get::METHOD.route().to(self::user::get),
    )
    .route(
        pdf::user::List::PATH,
        pdf::user::List::METHOD.route().to(self::user::list),
    );
}
