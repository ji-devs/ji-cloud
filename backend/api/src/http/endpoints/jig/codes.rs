use actix_web::{
    web::{self, Data, Json, Query},
    HttpResponse,
};
use shared::{
    api::{endpoints::jig::codes, ApiEndpoint},
    domain::jig::codes::{JigCode, JigCodeListResponse, JigCodeSessionsListResponse},
};
use sqlx::PgPool;

use crate::{db, error, extractor::TokenUser};

/// Create a jig player session for the author, if one does not exist already.
pub async fn create(
    db: Data<PgPool>,
    claims: TokenUser,
    req: Json<<codes::Create as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::JigCode> {
    let req = req.into_inner();
    let user_id = claims.user_id();

    db::jig::is_logged_in(&*db, user_id).await?;

    let jig_code = db::jig::codes::create(&db, user_id, &req).await?;

    Ok(HttpResponse::Created().json(jig_code))
}

pub async fn update(
    db: Data<PgPool>,
    claims: TokenUser,
    path: web::Path<JigCode>,
    req: Json<<codes::Update as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::JigCode> {
    let code = path.into_inner();
    let req = req.into_inner();
    let user_id = claims.user_id();

    db::jig::is_users_code(&*db, user_id, code).await?;

    db::jig::codes::update(&db, code, &req).await?;

    Ok(HttpResponse::NoContent().into())
}

/// Get all jig codes for user.
pub async fn list_user_codes(
    db: Data<PgPool>,
    claims: TokenUser,
    query: Option<Query<<codes::JigCodeList as ApiEndpoint>::Req>>,
) -> Result<Json<<codes::JigCodeList as ApiEndpoint>::Res>, error::JigCode> {
    let user_id = claims.user_id();
    let query = query.map_or_else(Default::default, Query::into_inner);

    let codes = db::jig::codes::list_user_codes(&*db, user_id, query).await?;

    Ok(Json(JigCodeListResponse { codes }))
}

/// Fetch all sessions for a code.
pub async fn list_code_sessions(
    db: Data<PgPool>,
    claims: TokenUser,
    path: web::Path<JigCode>,
) -> Result<Json<<codes::JigCodeSessions as ApiEndpoint>::Res>, error::JigCode> {
    let code = path.into_inner();
    let user_id = claims.user_id();

    let sessions = db::jig::codes::list_code_sessions(&*db, user_id, code).await?;

    Ok(Json(JigCodeSessionsListResponse { sessions }))
}

pub mod instance {
    use actix_web::{
        web::{Data, Json},
        HttpResponse,
    };
    use chrono::{Duration, Utc};
    use ji_core::settings::RuntimeSettings;
    use serde::Deserialize;
    use shared::{
        api::{endpoints::jig::codes, ApiEndpoint},
        domain::jig::codes::instance::PlayerSessionInstanceResponse,
    };
    use sqlx::PgPool;

    use crate::{
        db, error,
        extractor::IPAddress,
        token::{create_player_session_instance_token, validate_token},
    };
    use uuid::Uuid;

    /// Create a jig player session instance
    pub async fn start_session(
        settings: Data<RuntimeSettings>,
        db: Data<PgPool>,
        ip_address: IPAddress,
        req: Json<<codes::instance::Create as ApiEndpoint>::Req>,
    ) -> Result<
        (
            Json<<codes::instance::Create as ApiEndpoint>::Res>,
            actix_web::http::StatusCode,
        ),
        error::JigCode,
    > {
        let req = req.into_inner();

        let resp = db::jig::codes::start_session(&*db, req.code, ip_address).await?;

        let token: String = create_player_session_instance_token(
            &settings.token_secret,
            Duration::weeks(2),
            &resp.2,
            Utc::now(),
        )?;

        Ok((
            Json(PlayerSessionInstanceResponse {
                jig_id: resp.0,
                settings: resp.1,
                token,
            }),
            actix_web::http::StatusCode::CREATED,
        ))
    }

    #[derive(Deserialize)]
    struct InstanceToken {
        /// The instance this token is for.
        pub sub: Uuid,
    }

    /// Create a jig player session for someone who's not the author, if one doesn't already exist
    pub async fn complete_session(
        settings: Data<RuntimeSettings>,
        db: Data<PgPool>,
        ip_address: IPAddress,
        req: Json<<codes::instance::Complete as ApiEndpoint>::Req>,
    ) -> Result<HttpResponse, error::JigCode> {
        let req = req.into_inner();

        let token = validate_token(&req.token, None, &settings.token_secret)
            .map_err(|_| error::JigCode::Forbidden)?;

        let instance_token: InstanceToken = serde_json::from_value(token)?;

        db::jig::codes::complete_session(
            &db,
            req.session,
            req.players_name,
            instance_token.sub,
            ip_address,
        )
        .await?;

        Ok(HttpResponse::NoContent().finish())
    }
}
