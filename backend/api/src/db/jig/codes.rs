use chrono::{DateTime, Duration, Utc};
use rand::{rngs::ThreadRng, Rng};
use shared::config::{JIG_PLAYER_SESSION_CODE_MAX, JIG_PLAYER_SESSION_VALID_DURATION_SECS};
use shared::domain::jig::codes::{JigCodeSessionResponse, JigPlayerSessionCreateRequest};
use shared::domain::jig::{
    codes::{JigCode, JigCodeResponse, JigPlaySession},
    player::JigPlayerSettings,
    JigId,
};
use shared::domain::user::UserId;
use sqlx::{error::DatabaseError, postgres::PgDatabaseError, PgPool};
use uuid::Uuid;

use shared::domain::jig::TextDirection;

use crate::error;
use crate::extractor::IPAddress;

pub async fn create(
    db: &PgPool,
    creator_id: UserId,
    opts: &JigPlayerSessionCreateRequest,
) -> Result<(JigCode, DateTime<Utc>), error::JigCode> {
    let mut generator = rand::thread_rng();

    let mut code = generate_random_code(&mut generator);

    let expires_at = Utc::now() + Duration::seconds(JIG_PLAYER_SESSION_VALID_DURATION_SECS as i64);

    // retry as many times as there are possible codes
    // NOTE: this is NOT guaranteed to successfully insert if there
    for _ in 0..JIG_PLAYER_SESSION_CODE_MAX * 2 {
        match sqlx::query!(
            //language=SQL
            r#"
insert into jig_code (jig_id, creator_id, name, code, direction, display_score, track_assessments, drag_assist, expires_at)
values ($1, $2, $3, $4, $5, $6, $7, $8, $9)

"#,
            opts.jig_id.0,
            creator_id.0,
            opts.name,
            code,
            opts.settings.direction as i16,
            opts.settings.display_score,
            opts.settings.track_assessments,
            opts.settings.drag_assist,
            expires_at,
        )
        .execute(db)
        .await
        {
            Ok(_) => { // insert successful
                return Ok((JigCode(code), expires_at));
            },
            Err(err) => match err {
                sqlx::Error::Database(db_err) => {
                    session_create_error_or_continue(db_err)?;
                    // did not return error on previous line, retry with new code
                    code = generate_random_code(&mut generator);
                },
                err => return Err(anyhow::anyhow!("sqlx error: {:?}", err).into()),
            },
        }
    }

    Err(anyhow::anyhow!("Maximum retries reached for creating a new jig session").into())
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

pub async fn list_user_codes(db: &PgPool, user_id: UserId) -> sqlx::Result<Vec<JigCodeResponse>> {
    let sessions = sqlx::query!(
        //language=SQL
        r#"
select code     as "code!: i32",
       direction as "direction: TextDirection",
       display_score,
       track_assessments,
       drag_assist,
       name as "name?",
       expires_at as "expires_at: DateTime<Utc>"
from jig_code
where creator_id = $1
"#,
        user_id.0
    )
    .fetch_all(db)
    .await?
    .into_iter()
    .map(|it| JigCodeResponse {
        index: JigCode(it.code),
        name: it.name,
        settings: JigPlayerSettings {
            direction: it.direction,
            display_score: it.display_score,
            track_assessments: it.track_assessments,
            drag_assist: it.drag_assist,
        },
        expires_at: it.expires_at,
    })
    .collect();

    Ok(sessions)
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
               display_score, 
               track_assessments, 
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
            display_score: session_info.display_score,
            track_assessments: session_info.track_assessments,
            drag_assist: session_info.drag_assist,
        },
        instance_id,
    ))
}

/// Completes a jig player session for a player and updates play count
pub async fn complete_session(
    db: &PgPool,
    session: JigPlaySession,
    instance_id: Uuid,
    ip_address: IPAddress,
) -> Result<(), error::JigCode> {
    let session = serde_json::to_value(&session)?;
    sqlx::query!(
        //language=SQL
        r#"
            UPDATE jig_code_session
            SET finished_at = current_timestamp, info=$1
            WHERE id = $2 and ip_address = $3 and finished_at is null;
        "#,
        session,
        instance_id,
        ip_address.0,
    )
    .execute(db)
    .await?;

    Ok(())
}
