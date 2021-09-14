use actix_web::{
    web::{self, Data, Json},
    HttpResponse,
};
use shared::{
    api::{endpoints::jig::player, ApiEndpoint},
    domain::jig::{
        player::{JigPlayerSession, JigPlayerSessionListResponse},
        JigId,
    },
};
use sqlx::PgPool;

use crate::{db, error, extractor::TokenUser};

/// Create a jig player session for the author, if one does not exist already.
pub async fn create(
    db: Data<PgPool>,
    claims: TokenUser,
    req: Json<<player::Create as ApiEndpoint>::Req>,
) -> Result<HttpResponse, error::JigCode> {
    let req = req.into_inner();

    // let settings = req.settings.clone(); // FIXME get rid of this clone

    db::jig::authz(&*db, claims.0.user_id, Some(req.jig_id.clone())).await?;

    let index = db::jig::player::create(&db, req.jig_id, &req.settings).await?;

    Ok(HttpResponse::Created().json(JigPlayerSession {
        index,
        settings: req.settings,
    }))
}

/// Fetch a jig player session code from it's jig if it exists.
pub async fn list(
    db: Data<PgPool>,
    _claims: TokenUser,
    path: web::Path<JigId>,
) -> Result<Json<<player::List as ApiEndpoint>::Res>, error::JigCode> {
    let id = path.into_inner();

    let sessions = db::jig::player::list_sessions(&*db, id).await?;

    Ok(Json(JigPlayerSessionListResponse { sessions }))
}

pub mod instance {
    use actix_web::{
        web::{Data, Json},
        HttpResponse,
    };
    use chrono::{Duration, Utc};
    use core::settings::RuntimeSettings;
    use serde::Deserialize;
    use shared::{
        api::{endpoints::jig::player, ApiEndpoint},
        domain::jig::player::instance::PlayerSessionInstanceResponse,
    };
    use sqlx::PgPool;

    use crate::{
        db, error,
        extractor::{IPAddress, UserAgent},
        token::{create_player_session_instance_token, validate_token},
    };
    use uuid::Uuid;

    /// Create a jig player session instance
    pub async fn create_session_instance(
        settings: Data<RuntimeSettings>,
        db: Data<PgPool>,
        ip_address: IPAddress,
        user_agent: UserAgent,
        req: Json<<player::instance::Create as ApiEndpoint>::Req>,
    ) -> Result<
        (
            Json<<player::instance::Create as ApiEndpoint>::Res>,
            actix_web::http::StatusCode,
        ),
        error::JigCode,
    > {
        let req = req.into_inner();

        let resp =
            db::jig::player::create_session_instance(&*db, req.index, ip_address, user_agent)
                .await?;

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
    pub async fn complete_session_instance(
        settings: Data<RuntimeSettings>,
        db: Data<PgPool>,
        ip_address: IPAddress,
        user_agent: UserAgent,
        req: Json<<player::instance::Complete as ApiEndpoint>::Req>,
    ) -> Result<HttpResponse, error::JigCode> {
        let req = req.into_inner();

        let token = validate_token(&req.token, None, &settings.token_secret)
            .map_err(|_| error::JigCode::Forbidden)?;

        let instance_token: InstanceToken = serde_json::from_value(token)?;

        db::jig::player::complete_session_instance(
            &db,
            req.jig_id,
            ip_address,
            user_agent,
            instance_token.sub,
        )
        .await?;

        Ok(HttpResponse::NoContent().finish())
    }
}
