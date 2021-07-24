use crate::error;
use config::JIG_CODE_NUMBER_OF_DIGITS;
use shared::domain::jig::JigId;
use sqlx::postgres::PgDatabaseError;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create(db: &PgPool, jig_id: JigId) -> Result<i16, error::JigCode> {
    let mut txn = db.begin().await?;

    let exists = sqlx::query!(
        r#"select exists(select 1 from jig_code where jig_id=$1) as "exists!""#,
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
            r#"insert into jig_code (jig_id, index) values ($1, $2)"#,
            jig_id.0,
            index
        )
        .execute(&mut txn)
        .await
        {
            Ok(_) => return Ok(index), // insert successful
            Err(err) => match err {
                sqlx::Error::Database(db_err) => {
                    let constraint = db_err.downcast_ref::<PgDatabaseError>().constraint();
                    match constraint {
                        Some("jig_code_jig_id_key") => {
                            // code exists for the jig!
                            txn.rollback().await?;
                            return Err(error::JigCode::Conflict);
                        }
                        Some("jig_code_index_key") => {
                            // same code but different id. retry insert with a new code
                            index = rehash(index, n);
                            n += 1;
                        }
                        Some("jig_code_jig_id_fkey") => {
                            txn.rollback().await?;
                            return Err(error::JigCode::ResourceNotFound);
                        }
                        db_err => {
                            txn.rollback().await?;
                            return Err(anyhow::anyhow!(
                                "{}",
                                db_err.unwrap_or("unknown database error")
                            )
                            .into());
                        }
                    }
                }
                err => return Err(err.into()),
            },
        }
    }
}

/// Hashes a Uuid by
/// 1. XORing every 2 bytes together as an i16,
/// 2. clamping to within the digit requirement using mod (4 digits here),
/// 3. taking the absolute value to get rid of negative numbers
fn hash_id_to_code(id: Uuid) -> i16 {
    let hash = id
        .as_bytes()
        .windows(2)
        .fold(0, |acc, w| acc ^ (w[0] as i16 + (w[1] as i16) << 8));

    let result = (hash % 10_i16.pow(JIG_CODE_NUMBER_OF_DIGITS)) as i16;

    result.abs()
}

/// Rehash by
/// 1. adding 2.pow(attempt_number) to the previous conflicting hash:
///      hash(j) = hash(j-1) + 2.pow(j-1),
/// 2. clamping to within the digit requirement using mod (4 digits here),
/// 3. taking the absolute value to get rid of negative numbers
fn rehash(hash: i16, attempt: i16) -> i16 {
    (hash + (1 << attempt)) % 10_i16.pow(JIG_CODE_NUMBER_OF_DIGITS).abs()
}

pub async fn get(db: &PgPool, jig_id: JigId) -> anyhow::Result<Option<i16>> {
    let index = sqlx::query!(
        r#"select index as "index!: i16" from jig_code where jig_id = $1"#,
        jig_id.0
    )
    .fetch_optional(db)
    .await?
    .map(|it| it.index);

    Ok(index)
}

pub async fn get_jig_from_code(db: &PgPool, index: i16) -> anyhow::Result<Option<JigId>> {
    let id: Option<JigId> = sqlx::query!(
        r#"select jig_id as "id!: JigId" from jig_code where index = $1"#,
        index
    )
    .fetch_optional(db)
    .await?
    .map(|it| it.id);

    log::info!("{}", id.unwrap().0);

    Ok(id)
}
