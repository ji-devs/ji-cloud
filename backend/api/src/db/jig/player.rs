use chrono::{DateTime, Duration, Utc};
use rand::{rngs::ThreadRng, Rng};
use shared::config::{JIG_PLAYER_SESSION_CODE_MAX, JIG_PLAYER_SESSION_VALID_DURATION_SECS};
use shared::domain::jig::{
    player::{JigPlayerSession, JigPlayerSessionIndex, JigPlayerSettings},
    JigId, TextDirection,
};
use sqlx::{error::DatabaseError, postgres::PgDatabaseError, PgPool};
use uuid::Uuid;

use crate::{
    error,
    extractor::{IPAddress, UserAgent},
};

pub async fn create(
    db: &PgPool,
    jig_id: JigId,
    settings: &JigPlayerSettings,
) -> Result<(JigPlayerSessionIndex, DateTime<Utc>), error::JigCode> {
    let mut generator = rand::thread_rng();

    let mut index = generate_random_code(&mut generator);

    let expires_at = Utc::now() + Duration::seconds(JIG_PLAYER_SESSION_VALID_DURATION_SECS as i64);

    // retry as many times as there are possible codes
    // NOTE: this is NOT guaranteed to successfully insert if there
    for _ in 0..JIG_PLAYER_SESSION_CODE_MAX * 2 {
        log::debug!("Try insert with index {}", index);
        match sqlx::query!(
            //language=SQL
            r#"
insert into jig_player_session (jig_id, index, direction, display_score, track_assessments, drag_assist, expires_at)
values ($1, $2, $3, $4, $5, $6, $7)

"#,
            jig_id.0,
            index,
            settings.direction as i16,
            settings.display_score,
            settings.track_assessments,
            settings.drag_assist,
            expires_at,
        )
        .execute(db)
        .await
        {
            Ok(_) => { // insert successful
                return Ok((JigPlayerSessionIndex(index), expires_at));
            },
            Err(err) => match err {
                sqlx::Error::Database(db_err) => {
                    session_create_error_or_continue(db_err)?;
                    // did not return error on previous line, retry with new index
                    index = generate_random_code(&mut generator);
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

fn generate_random_code(generator: &mut ThreadRng) -> i16 {
    debug_assert!(JIG_PLAYER_SESSION_CODE_MAX > 0);

    generator.gen_range(0..JIG_PLAYER_SESSION_CODE_MAX)
}

pub async fn list_sessions(db: &PgPool, jig_id: JigId) -> sqlx::Result<Vec<JigPlayerSession>> {
    let sessions = sqlx::query!(
        //language=SQL
        r#"
select index     as "index!: i16",
       direction as "direction: TextDirection",
       display_score,
       track_assessments,
       drag_assist,
       expires_at as "expires_at: DateTime<Utc>"
from jig_player_session
where jig_id = $1
"#,
        jig_id.0
    )
    .fetch_all(db)
    .await?
    .into_iter()
    .map(|it| JigPlayerSession {
        index: JigPlayerSessionIndex(it.index),
        settings: JigPlayerSettings {
            direction: it.direction,
            display_score: it.display_score,
            track_assessments: it.track_assessments,
            drag_assist: it.drag_assist,
        },
        expires_at: it.expires_at,
    })
    .collect();

    log::info!("{:?}", sessions);

    Ok(sessions)
}

/// Creates new jig player session for a player
pub async fn create_session_instance(
    db: &PgPool,
    session_index: JigPlayerSessionIndex,
    ip_address: IPAddress,
    user_agent: UserAgent,
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
        from jig_player_session
        where index=$1
        "#,
        session_index.0
    )
    .fetch_optional(&mut txn)
    .await?
    .ok_or(error::JigCode::ResourceNotFound)?;

    // insert into the jig_player_session_instance table returning the instance_id
    let instance_id = sqlx::query!(
        //language=SQL
        r#"
        insert into jig_player_session_instance (session_index, ip_address, user_agent)
        values ($1, $2, $3)
        returning id as "id: Uuid"
        "#,
        session_index.0,
        ip_address.0,
        user_agent.0
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
pub async fn complete_session_instance(
    db: &PgPool,
    ip_address: IPAddress,
    user_agent: UserAgent,
    instance_id: Uuid,
) -> Result<(), error::JigCode> {
    let mut txn = db.begin().await?;

    let resp = sqlx::query!(
        //language=SQL
        r#"
delete
from jig_player_session_instance
where id = $1
returning ip_address, user_agent, (
    select jig_id
    from jig_player_session_instance
             join jig_player_session on session_index = index
) as "jig_id!: JigId"
        "#,
        instance_id,
    )
    .fetch_optional(&mut txn)
    .await?
    .ok_or(error::JigCode::ResourceNotFound)?;

    // FIXME
    if (resp.user_agent).ne(&user_agent.0) | (resp.ip_address).ne(&ip_address.0) {
        return Err(error::JigCode::ResourceNotFound);
    }

    sqlx::query!(
        //language=SQL
        r#"
        update jig_play_count
        set play_count = play_count + 1
        where jig_id = $1
        "#,
        resp.jig_id.0,
    )
    .execute(db)
    .await?;

    txn.commit().await?;

    Ok(())
}
