use crate::error;
use crate::extractor::IPAddress;
use core::config::JIG_PLAYER_SESSION_CODE_MAX;
use shared::domain::jig::player::JigPlayerSettings;
use shared::domain::jig::{JigId, TextDirection};
use sqlx::postgres::PgDatabaseError;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create(
    db: &PgPool,
    jig_id: JigId,
    settings: JigPlayerSettings,
) -> Result<i16, error::JigCode> {
    let mut txn = db.begin().await?;

    let exists = sqlx::query!(
        r#"select exists(select 1 from jig_player_session where jig_id=$1) as "exists!""#,
        jig_id.0
    )
    .fetch_one(&mut txn)
    .await?
    .exists;
    if exists {
        txn.rollback().await?;
        return Err(error::JigCode::Conflict);
    }

    // retry until successful:
    let mut index = hash_id_to_code(jig_id.0);
    let mut n: i16 = 0;

    loop {
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
        .execute(&mut txn)
        .await
        {
            Ok(_) => { // insert successful
                txn.commit().await?;
                return Ok(index);
            },
            Err(err) => match err {
                sqlx::Error::Database(db_err) => {
                    let constraint = db_err.downcast_ref::<PgDatabaseError>().constraint();
                    match constraint {
                        Some("jig_player_session_jig_id_key") => {
                            // session exists for the jig, prevents race condition?
                            log::info!("1");
                            txn.rollback().await?;
                            return Err(error::JigCode::Conflict);
                        }
                        Some("jig_player_session_pkey") => {
                            // same code but different jig. retry insert with a new code
                            log::info!("how? {}", &index);
                            index = rehash(index, &n);
                            n += 1;
                        }
                        Some("jig_player_session_jig_id_fkey") => {
                            // no jig with this id exists
                            log::info!("2");
                            txn.rollback().await?;
                            return Err(error::JigCode::ResourceNotFound);
                        }
                        db_err => {
                            txn.rollback().await?;
                            log::info!("3");
                            return Err(anyhow::anyhow!(
                                "{}",
                                db_err.unwrap_or("unknown database error")
                            )
                            .into());
                        }
                    }
                }
                err => return Err(anyhow::anyhow!("sqlx error: {:?}", err).into()),
            },
        }
    }
}

/// Creates new jig player session for a player
// TODO: Fixme
// TODO: Add Settings?
pub async fn create_user_session(
    db: &PgPool,
    session_index: i16,
    ip_addr: IPAddress,
) -> Result<String, error::JigCode> {
    let mut txn = db.begin().await?;

    let jig_id = sqlx::query!(
        //language=SQL
        r#"
        select jig_id
        from jig_player_session
        where index=$1
        "#,
        session_index
    )
    .fetch_optional(&mut txn)
    .await?
    .ok_or(error::JigCode::ResourceNotFound)?;

    // get the user agent header and ip address
    let ip_address = ip_addr.ip_addr.ok_or(error::JigCode::ResourceNotFound)?;
    let user_agent = ip_addr.user_agent.ok_or(error::JigCode::ResourceNotFound)?;

    // insert into the jig_player_session_instance table returning the instance_id
    let jig_player_session_instance_id = sqlx::query!(
        //language=SQL
        r#"
        insert into jig_player_session_instance (index, jig_id, ip_addr, user_agent)
        values ($1, $2, $3, $4)
        returning instance_id as "id: Uuid"
        "#,
        session_index,
        jig_id.jig_id,
        ip_address,
        user_agent
    )
    .fetch_one(&mut txn)
    .await?;

    txn.commit().await?;

    Ok(jig_player_session_instance_id.id.to_string())
}

/// Completes a jig player session for a player and updates play count
pub async fn complete_user_session(
    db: &PgPool,
    jig_id: JigId,
    ip_addr: IPAddress,
    session_id: &str,
) -> Result<(), error::JigCode> {
    let txn = db.begin().await?;

    let instance_id = Uuid::parse_str(&session_id).unwrap();

    let jig_session_index_query = sqlx::query!(
        //language=SQL
        r#"
        select *
        from jig_player_session_instance
        where instance_id = $1
        "#,
        instance_id,
    )
    .fetch_optional(db)
    .await?
    .ok_or(error::JigCode::ResourceNotFound)?;

    // get the user agent header and ip address and check validity
    let ip_address = ip_addr.ip_addr.ok_or(error::JigCode::ResourceNotFound)?;
    let user_agent = ip_addr.user_agent.ok_or(error::JigCode::ResourceNotFound)?;

    // unwrap because invalid ip/user agents are checked before putting them into the db
    // confirms user agent and ip address match
    if (jig_session_index_query.user_agent.unwrap()).ne(&user_agent)
        | (jig_session_index_query.ip_addr.unwrap()).ne(&ip_address)
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

    result.abs()
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

pub async fn get(db: &PgPool, index: i16) -> anyhow::Result<Option<(JigId, JigPlayerSettings)>> {
    let res = sqlx::query!(
        //language=SQL
        r#"
select jig_id as "id: JigId",
       direction as "direction: TextDirection",
       display_score, 
       track_assessments, 
       drag_assist
from jig_player_session 
where index = $1
        "#,
        index
    )
    .fetch_optional(db)
    .await?
    .map(|it| {
        (
            it.id,
            JigPlayerSettings {
                direction: it.direction,
                display_score: it.display_score,
                track_assessments: it.track_assessments,
                drag_assist: it.drag_assist,
            },
        )
    });

    Ok(res)
}

pub async fn get_code(db: &PgPool, jig_id: JigId) -> anyhow::Result<Option<i16>> {
    log::info!("asdasda {:?}", &jig_id);
    let index = sqlx::query!(
        r#"select index as "index!: i16" from jig_player_session where jig_id = $1"#,
        jig_id.0
    )
    .fetch_optional(db)
    .await?
    .map(|it| it.index);

    log::info!("{:?}", index);

    Ok(index)
}
