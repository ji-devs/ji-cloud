use chrono::{DateTime, Duration, Utc};
use futures::future::join_all;
use rand::{rngs::ThreadRng, Rng};
use serde_json::value::Value;
use shared::config::{JIG_PLAYER_SESSION_CODE_MAX, JIG_PLAYER_SESSION_VALID_DURATION_SECS};
use shared::domain::additional_resource::{AdditionalResource, ResourceContent};
use shared::domain::asset::DraftOrLive;
use shared::domain::jig::codes::{
    JigCodeListRequest, JigCodeSessionResponse, JigCodeUpdateRequest,
    JigPlayerSessionCreateRequest, JigWithCodes,
};
use shared::domain::jig::{
    codes::{JigCode, JigCodeResponse, JigPlaySession},
    player::JigPlayerSettings,
    JigId,
};
use shared::domain::module::LiteModule;
use shared::domain::user::UserId;
use shared::domain::{
    additional_resource::AdditionalResourceId as AddId,
    asset::PrivacyLevel,
    category::CategoryId,
    jig::{AudioBackground, AudioFeedbackNegative, AudioFeedbackPositive, JigRating},
    meta::ResourceTypeId as TypeId,
    meta::{AffiliationId, AgeRangeId},
    module::{body::ThemeId, ModuleId, ModuleKind, StableModuleId},
};
use sqlx::{error::DatabaseError, postgres::PgDatabaseError, types::Json, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

use shared::domain::jig::{AudioEffects, JigAdminData, JigData, JigResponse, TextDirection};

use crate::error;
use crate::extractor::IPAddress;

pub async fn create(
    db: &PgPool,
    creator_id: UserId,
    opts: &JigPlayerSessionCreateRequest,
) -> Result<JigCodeResponse, error::JigCode> {
    let mut generator = rand::thread_rng();

    let mut code = generate_random_code(&mut generator);

    let expires_at = Utc::now() + Duration::seconds(JIG_PLAYER_SESSION_VALID_DURATION_SECS as i64);

    // retry as many times as there are possible codes
    // NOTE: this is NOT guaranteed to successfully insert if there
    for _ in 0..JIG_PLAYER_SESSION_CODE_MAX * 2 {
        match sqlx::query!(
            //language=SQL
            r#"
insert into jig_code (jig_id, creator_id, name, code, direction, scoring, drag_assist, expires_at)
values ($1, $2, $3, $4, $5, $6, $7, $8)
returning created_at as "created_at: DateTime<Utc>"
"#,
            opts.jig_id.0,
            creator_id.0,
            opts.name,
            code,
            opts.settings.direction as i16,
            opts.settings.scoring,
            opts.settings.drag_assist,
            expires_at,
        )
        .fetch_one(db)
        .await
        {
            Ok(res) => {
                return Ok(JigCodeResponse {
                    index: JigCode(code),
                    jig_id: opts.jig_id,
                    name: opts.name.clone(),
                    settings: opts.settings.clone(),
                    created_at: res.created_at,
                    expires_at,
                })
            }
            Err(err) => match err {
                sqlx::Error::Database(db_err) => {
                    session_create_error_or_continue(db_err)?;
                    // did not return error on previous line, retry with new code
                    code = generate_random_code(&mut generator);
                }
                err => return Err(anyhow::anyhow!("sqlx error: {:?}", err).into()),
            },
        }
    }

    Err(anyhow::anyhow!("Maximum retries reached for creating a new jig session").into())
}

pub async fn update(
    db: &PgPool,
    code: JigCode,
    opts: &JigCodeUpdateRequest,
) -> Result<(), error::JigCode> {
    let name = opts.name.clone();
    let direction = opts.settings.as_ref().map(|opts| opts.direction);
    let scoring = opts.settings.as_ref().map(|opts| opts.scoring);
    let drag_assist = opts.settings.as_ref().map(|opts| opts.drag_assist);

    sqlx::query!(
        //language=SQL
        r#"
            update jig_code
            set name = case when $2 then $3 else name end,
                direction = coalesce($4, direction),
                scoring = coalesce($5, scoring),
                drag_assist = coalesce($6, drag_assist)
            where code = $1
        "#,
        code.0,
        name.is_some(),
        name.flatten(),
        direction.map(|d| d as i16),
        scoring,
        drag_assist,
    )
    .execute(db)
    .await?;

    Ok(())
}

fn session_create_error_or_continue(db_err: Box<dyn DatabaseError>) -> Result<(), error::JigCode> {
    let constraint = db_err.downcast_ref::<PgDatabaseError>().constraint();

    match constraint {
        Some("jig_player_session_pkey") => {
            // same code but different jig. retry insert with a new code
            Ok(())
        }
        Some("jig_player_session_jig_id_fkey") => {
            // no jig with this id exists
            Err(error::JigCode::ResourceNotFound)
        }
        db_err => Err(anyhow::anyhow!("{}", db_err.unwrap_or("unknown database error")).into()),
    }
}

fn generate_random_code(generator: &mut ThreadRng) -> i32 {
    debug_assert!(JIG_PLAYER_SESSION_CODE_MAX > 0);

    generator.gen_range(0..JIG_PLAYER_SESSION_CODE_MAX)
}

pub async fn get_code(db: &PgPool, code: JigCode) -> sqlx::Result<JigCodeResponse> {
    let row = sqlx::query!(
        //language=SQL
        r#"
            select code as "code!: i32",
                jig_id as "jig_id: JigId",
                direction as "direction: TextDirection",
                scoring,
                drag_assist,
                name as "name?",
                created_at as "created_at: DateTime<Utc>",
                expires_at as "expires_at: DateTime<Utc>"
            from jig_code
            where code = $1
        "#,
        code.0,
    )
    .fetch_one(db)
    .await?;

    let response = JigCodeResponse {
        index: JigCode(row.code),
        jig_id: row.jig_id,
        name: row.name,
        settings: JigPlayerSettings {
            direction: row.direction,
            scoring: row.scoring,
            drag_assist: row.drag_assist,
        },
        created_at: row.created_at,
        expires_at: row.expires_at,
    };

    Ok(response)
}

pub async fn list_user_codes(
    db: &PgPool,
    user_id: UserId,
    query: JigCodeListRequest,
) -> sqlx::Result<Vec<JigCodeResponse>> {
    let codes = sqlx::query!(
        //language=SQL
        r#"
select code     as "code!: i32",
       jig_id as "jig_id: JigId",
       direction as "direction: TextDirection",
       scoring,
       drag_assist,
       name as "name?",
       created_at as "created_at: DateTime<Utc>",
       expires_at as "expires_at: DateTime<Utc>"
from jig_code
where creator_id = $1 AND (jig_id = $2 or $2 is null)
order by created_at desc
"#,
        user_id.0,
        query.jig_id.map(|j| j.0),
    )
    .fetch_all(db)
    .await?
    .into_iter()
    .map(|it| JigCodeResponse {
        index: JigCode(it.code),
        jig_id: it.jig_id,
        name: it.name,
        settings: JigPlayerSettings {
            direction: it.direction,
            scoring: it.scoring,
            drag_assist: it.drag_assist,
        },
        created_at: it.created_at,
        expires_at: it.expires_at,
    })
    .collect();

    Ok(codes)
}

pub async fn jigs_with_codes(db: &PgPool, user_id: UserId) -> sqlx::Result<Vec<JigWithCodes>> {
    let jigs = sqlx::query!(
        //language=SQL
        r#"
            with cte as (
                select id as "jig_id",
                    creator_id,
                    author_id,
                    liked_count,
                    play_count,
                    live_up_to_date,
                    jig.live_id,
                    published_at,
                    rating,
                    blocked,
                    curated,
                    is_premium
                from jig
                left join jig_play_count on jig_play_count.jig_id = jig.id
                left join jig_admin_data "admin" on admin.jig_id = jig.id
            )
            select
                cte.jig_id                                         as "jig_id: JigId",
                display_name,
                max(jig_code.created_at) as last_code_created_at,
                cte.creator_id                                     as "creator_id: UserId",
                cte.author_id                                      as "author_id: UserId",
                (select given_name || ' '::text || family_name
                from user_profile
                where user_profile.user_id = author_id)            as "author_name",
                jig_data.created_at,
                jig_data.updated_at,
                cte.published_at,
                jig_data.privacy_level                             as "privacy_level!: PrivacyLevel",
                jig_data.language,
                jig_data.description,
                jig_data.translated_description                    as "translated_description!: Json<HashMap<String, String>>",
                jig_data.direction                                 as "direction: TextDirection",
                jig_data.scoring,
                jig_data.drag_assist,
                jig_data.theme                                     as "theme: ThemeId",
                jig_data.audio_background                          as "audio_background: AudioBackground",
                cte.liked_count,
                cte.play_count,
                cte.live_up_to_date,
                exists(select 1 from jig_like where user_id = $1)    as "is_liked!",
                jig_data.locked,
                jig_data.other_keywords,
                jig_data.translated_keywords,
                cte.rating                                         as "rating?: JigRating",
                cte.blocked                                        as "blocked",
                cte.curated,
                cte.is_premium                                     as "premium",
                array(select row (unnest(audio_feedback_positive))) as "audio_feedback_positive!: Vec<(AudioFeedbackPositive,)>",
                array(select row (unnest(audio_feedback_negative))) as "audio_feedback_negative!: Vec<(AudioFeedbackNegative,)>",
                array(
                    select row (jig_data_module.id, jig_data_module.stable_id, kind, is_complete)
                    from jig_data_module
                    where jig_data_id = jig_data.id
                    order by "index"
                ) as "modules!: Vec<(ModuleId, StableModuleId, ModuleKind, bool)>",
                array(select row (category_id)
                    from jig_data_category
                    where jig_data_id = cte.live_id)     as "categories!: Vec<(CategoryId,)>",
                array(select row (affiliation_id)
                    from jig_data_affiliation
                    where jig_data_id = cte.live_id)     as "affiliations!: Vec<(AffiliationId,)>",
                array(select row (age_range_id)
                    from jig_data_age_range
                    where jig_data_id = cte.live_id)     as "age_ranges!: Vec<(AgeRangeId,)>",
                array(
                    select row (jdar.id, jdar.display_name, resource_type_id, resource_content)
                    from jig_data_additional_resource "jdar"
                    where jdar.jig_data_id = cte.live_id
                ) as "additional_resource!: Vec<(AddId, String, TypeId, Value)>"
            from jig_data
                inner join cte on cte.live_id = jig_data.id
                inner join jig_code on cte.jig_id = jig_code.jig_id
            group by cte.jig_id, display_name, cte.creator_id, cte.author_id, author_id, author_name, updated_at, published_at, privacy_level, language, description, translated_description, theme, audio_background, liked_count, play_count, live_up_to_date, locked, other_keywords, translated_keywords, rating, blocked, curated, premium, audio_feedback_positive, audio_feedback_negative, jig_data.created_at, jig_data.updated_at, jig_data.direction, jig_data.scoring, jig_data.drag_assist, "modules!: Vec<(ModuleId, StableModuleId, ModuleKind, bool)>", "categories!: Vec<(CategoryId,)>", "affiliations!: Vec<(AffiliationId,)>", "age_ranges!: Vec<(AgeRangeId,)>", "additional_resource!: Vec<(AddId, String, TypeId, Value)>"
            order by last_code_created_at desc
        "#,
        user_id.0
    )
    .fetch_all(db)
    .await?
    .into_iter()
    .map(|row| {
        async move {
            let codes = sqlx::query!(
                //language=SQL
                r#"
                    select code as "code!: i32",
                        jig_id as "jig_id: JigId",
                        direction as "direction: TextDirection",
                        scoring,
                        drag_assist,
                        name as "name?",
                        created_at as "created_at: DateTime<Utc>",
                        expires_at as "expires_at: DateTime<Utc>"
                    from jig_code
                    where jig_id = $1
                    order by created_at desc
                "#,
                row.jig_id.0
            )
            .fetch_all(db)
            .await?
            .into_iter()
            .map(|row| {
                JigCodeResponse {
                    index: JigCode(row.code),
                    jig_id: row.jig_id,
                    name: row.name,
                    settings: JigPlayerSettings {
                        direction: row.direction,
                        scoring: row.scoring,
                        drag_assist: row.drag_assist,
                    },
                    created_at: row.created_at,
                    expires_at: row.expires_at,
                }
            }).collect();

            let jig = JigResponse {
                id: row.jig_id,
                published_at: row.published_at,
                creator_id: row.creator_id,
                author_id: row.author_id,
                author_name: row.author_name,
                likes: row.liked_count,
                plays: row.play_count,
                live_up_to_date: row.live_up_to_date,
                is_liked: row.is_liked,
                jig_data: JigData {
                    created_at: row.created_at,
                    draft_or_live: DraftOrLive::Live,
                    display_name: row.display_name,
                    language: row.language,
                    modules: row
                        .modules
                        .into_iter()
                        .map(|(id, stable_id, kind, is_complete)| LiteModule {
                            id,
                            stable_id,
                            kind,
                            is_complete,
                        })
                        .collect(),
                    categories: row.categories.into_iter().map(|(it,)| it).collect(),
                    last_edited: row.updated_at,
                    description: row.description,
                    default_player_settings: JigPlayerSettings {
                        direction: row.direction,
                        scoring: row.scoring,
                        drag_assist: row.drag_assist,
                    },
                    theme: row.theme,
                    age_ranges: row.age_ranges.into_iter().map(|(it,)| it).collect(),
                    affiliations: row.affiliations.into_iter().map(|(it,)| it).collect(),
                    additional_resources: row
                        .additional_resource
                        .into_iter()
                        .map(
                            |(id, display_name, resource_type_id, resource_content)| AdditionalResource {
                                id,
                                display_name,
                                resource_type_id,
                                resource_content: serde_json::from_value::<ResourceContent>(
                                    resource_content,
                                )
                                .unwrap(),
                            },
                        )
                        .collect(),
                    audio_background: row.audio_background,
                    audio_effects: AudioEffects {
                        feedback_positive: row
                            .audio_feedback_positive
                            .into_iter()
                            .map(|(it,)| it)
                            .collect(),
                        feedback_negative: row
                            .audio_feedback_negative
                            .into_iter()
                            .map(|(it,)| it)
                            .collect(),
                    },
                    privacy_level: row.privacy_level,
                    locked: row.locked,
                    other_keywords: row.other_keywords,
                    translated_keywords: row.translated_keywords,
                    translated_description: row.translated_description.0,
                },
                admin_data: JigAdminData {
                    rating: row.rating,
                    blocked: row.blocked,
                    curated: row.curated,
                    premium: row.premium,
                },
            };

            Ok(JigWithCodes {
                jig,
                codes,
            })
        }
    });
    let jigs = join_all(jigs)
        .await
        .into_iter()
        .collect::<Result<_, sqlx::Error>>()?;
    Ok(jigs)
}

pub async fn list_code_sessions(
    db: &PgPool,
    user_id: UserId,
    code: JigCode,
) -> Result<Vec<JigCodeSessionResponse>, error::JigCode> {
    // ensure this user's code
    sqlx::query!(
        //language=SQL
        r#"
            SELECT * FROM jig_code
            WHERE creator_id = $1 AND code = $2;
        "#,
        user_id.0,
        code.0
    )
    .fetch_one(db)
    .await?;

    let sessions = sqlx::query!(
        //language=SQL
        r#"
            SELECT
                id,
                code,
                players_name,
                started_at,
                finished_at,
                info
            FROM jig_code_session
            WHERE code = $1 AND finished_at IS NOT NULL
            ORDER BY started_at;
        "#,
        code.0
    )
    .fetch_all(db)
    .await?
    .into_iter()
    .map(|it| {
        Ok(JigCodeSessionResponse {
            code: JigCode(it.code),
            players_name: it.players_name,
            started_at: it.started_at,
            finished_at: it.finished_at,
            info: match it.info {
                Some(r) => serde_json::from_value(r)?,
                None => None,
            },
        })
    })
    .collect::<Result<Vec<_>, error::JigCode>>()?;

    Ok(sessions)
}

/// Creates new jig player session for a player
pub async fn start_session(
    db: &PgPool,
    code: JigCode,
    ip_address: IPAddress,
) -> Result<(JigId, JigPlayerSettings, Uuid), error::JigCode> {
    let mut txn = db.begin().await?;

    let session_info = sqlx::query!(
        //language=SQL
        r#"
        select jig_id as "jig_id: JigId", 
               direction as "direction: TextDirection", 
               scoring,
               drag_assist
        from jig_code
        where code=$1
        "#,
        code.0
    )
    .fetch_optional(&mut txn)
    .await?
    .ok_or(error::JigCode::ResourceNotFound)?;

    // insert into the jig_code_session table returning the instance_id
    let instance_id = sqlx::query!(
        //language=SQL
        r#"
        insert into jig_code_session (code, started_at, ip_address)
        values ($1, current_timestamp, $2)
        returning id as "id: Uuid"
        "#,
        code.0,
        ip_address.0,
    )
    .fetch_one(&mut txn)
    .await?
    .id;

    txn.commit().await?;

    Ok((
        session_info.jig_id,
        JigPlayerSettings {
            direction: session_info.direction,
            scoring: session_info.scoring,
            drag_assist: session_info.drag_assist,
        },
        instance_id,
    ))
}

/// Completes a jig player session for a player and updates play count
pub async fn complete_session(
    db: &PgPool,
    session: JigPlaySession,
    players_name: Option<String>,
    instance_id: Uuid,
    ip_address: IPAddress,
) -> Result<(), error::JigCode> {
    let session = serde_json::to_value(&session)?;
    sqlx::query!(
        //language=SQL
        r#"
            UPDATE jig_code_session
            SET finished_at = current_timestamp, info=$1, players_name=$2
            WHERE id = $3 and ip_address = $4 and finished_at is null;
        "#,
        session,
        players_name,
        instance_id,
        ip_address.0,
    )
    .execute(db)
    .await?;

    Ok(())
}
