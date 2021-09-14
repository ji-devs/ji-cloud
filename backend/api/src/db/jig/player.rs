use core::config::JIG_PLAYER_SESSION_CODE_MAX;
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
) -> Result<JigPlayerSessionIndex, error::JigCode> {
    // retry until successful:
    let mut index = hash_id_to_code(jig_id.0);
    let mut n: i16 = 0;

    loop {
        log::info!("try insert with index {}", index);
        match sqlx::query!(
            //language=SQL
            r#"
insert into jig_player_session (jig_id, index, direction, display_score, track_assessments, drag_assist)
values ($1, $2, $3, $4, $5, $6)

"#,
            jig_id.0,
            index,
            settings.direction as i16,
            settings.display_score,
            settings.track_assessments,
            settings.drag_assist,
        )
        .execute(db)
        .await
        {
            Ok(_) => { // insert successful
                log::info!("OK");
                return Ok(JigPlayerSessionIndex(index));
            },
            Err(err) => match err {
                sqlx::Error::Database(db_err) => {
                    session_create_error_or_continue(db_err)?;

                    // here means retry with new index
                    log::info!("how? {}", &index);
                    index = rehash(index, &n);
                    log::info!("newnew {}", &index);
                    n += 1;
                },
                err => return Err(anyhow::anyhow!("sqlx error: {:?}", err).into()),
            },
        }
    }
}

fn session_create_error_or_continue(db_err: Box<dyn DatabaseError>) -> Result<(), error::JigCode> {
    let constraint = db_err.downcast_ref::<PgDatabaseError>().constraint();

    match constraint {
        Some("jig_player_session_pkey") => {
            // same code but different jig. retry insert with a new code
            log::info!("1");

            Ok(())
        }
        Some("jig_player_session_jig_id_fkey") => {
            // no jig with this id exists
            log::info!("2");
            Err(error::JigCode::ResourceNotFound)
        }
        db_err => {
            log::info!("3");
            Err(anyhow::anyhow!("{}", db_err.unwrap_or("unknown database error")).into())
        }
    }
}

/// Hashes a Uuid by
/// 1. XORing every 2 bytes together as an i16,
/// 2. clamping to within the digit requirement using mod (4 digits here),
/// 3. taking the absolute value to get rid of negative numbers
fn hash_id_to_code(id: Uuid) -> i16 {
    let bytes_to_word = |a: &u8, b: &u8| *a as i16 + (*b as i16) << 8;

    let hash = id
        .as_bytes()
        .windows(2)
        .fold(0, |acc, w| acc ^ bytes_to_word(&w[0], &w[1]));

    let result = (hash % 10000) as i16;

    result.abs();

    // FIXME temp debug
    1234
}

/// Rehash by
/// 1. adding 2.pow(attempt_number) to the previous conflicting hash:
///      hash(j) = hash(j-1) + 2.pow(j-1),
/// 2. clamping to within the digit requirement using mod (4 digits here),
/// 3. taking the absolute value to get rid of negative numbers
#[inline]
fn rehash(hash: i16, attempt: &i16) -> i16 {
    ((hash + (1 << attempt)) % (JIG_PLAYER_SESSION_CODE_MAX + 1)).abs()
}

pub async fn list_sessions(db: &PgPool, jig_id: JigId) -> sqlx::Result<Vec<JigPlayerSession>> {
    let sessions = sqlx::query!(
        //language=SQL
        r#"
select index     as "index!: i16",
       direction as "direction: TextDirection",
       display_score,
       track_assessments,
       drag_assist
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
    jig_id: JigId,
    ip_address: IPAddress,
    user_agent: UserAgent,
    instance_id: Uuid,
) -> Result<(), error::JigCode> {
    let mut txn = db.begin().await?;

    let jig_session_index_query = sqlx::query!(
        //language=SQL
        r#"
        select ip_address, user_agent
        from jig_player_session_instance
        where id = $1
        "#,
        instance_id,
    )
    .fetch_optional(&mut txn)
    .await?
    .ok_or(error::JigCode::ResourceNotFound)?;

    // FIXME
    if (jig_session_index_query.user_agent).ne(&user_agent.0)
        | (jig_session_index_query.ip_address).ne(&ip_address.0)
    {
        return Err(error::JigCode::ResourceNotFound);
    }

    sqlx::query!(
        //language=SQL
        r#"
        update jig_play_count
        set play_count = play_count + 1
        where jig_id = $1
        "#,
        jig_id.0,
    )
    .execute(db)
    .await?;

    txn.commit().await?;

    Ok(())
}
