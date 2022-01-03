use chrono::{DateTime, Utc};
use shared::domain::jig::{
    curation::{
        CommentId, JigCurationComment, JigCurationData, JigCurationFieldsDone, JigCurationStatus,
    },
    JigId,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error;

pub async fn update(
    pool: &PgPool,
    jig_id: JigId,
    display_name: Option<bool>,
    goals: Option<bool>,
    categories: Option<bool>,
    age_ranges: Option<bool>,
    affiliations: Option<bool>,
    language: Option<bool>,
    description: Option<bool>,
    additional_resources: Option<bool>,
    curation_status: Option<JigCurationStatus>,
) -> anyhow::Result<(), error::Auth> {
    let mut txn = pool.begin().await?;

    if let Some(display_name) = display_name {
        sqlx::query!(
            //language=SQL
            r#"
update jig_curation_data
set display_name = $2
where jig_id = $1 and $2 is distinct from display_name
            "#,
            jig_id.0,
            display_name,
        )
        .execute(&mut txn)
        .await?;
    }

    if let Some(goals) = goals {
        sqlx::query!(
            //language=SQL
            r#"
update jig_curation_data
set goals = $2
where jig_id = $1 and $2 is distinct from goals
            "#,
            jig_id.0,
            goals,
        )
        .execute(&mut txn)
        .await?;
    }

    if let Some(language) = language {
        sqlx::query!(
            //language=SQL
            r#"
update jig_curation_data
set language = $2
where jig_id = $1 and $2 is distinct from language
            "#,
            jig_id.0,
            language,
        )
        .execute(&mut txn)
        .await?;
    }

    if let Some(categories) = categories {
        sqlx::query!(
            //language=SQL
            r#"
update jig_curation_data
set categories = $2
where jig_id = $1 and $2 is distinct from categories
            "#,
            jig_id.0,
            categories,
        )
        .execute(&mut txn)
        .await?;
    }

    if let Some(description) = description {
        sqlx::query!(
            //language=SQL
            r#"
    update jig_curation_data
    set description = $2
    where jig_id = $1 and $2 is distinct from description
                "#,
            jig_id.0,
            description,
        )
        .execute(&mut txn)
        .await?;
    }

    if let Some(age_ranges) = age_ranges {
        sqlx::query!(
            //language=SQL
            r#"
    update jig_curation_data
    set age_ranges = $2
    where jig_id = $1 and $2 is distinct from age_ranges
                "#,
            jig_id.0,
            age_ranges,
        )
        .execute(&mut txn)
        .await?;
    }

    if let Some(affiliations) = affiliations {
        sqlx::query!(
            //language=SQL
            r#"
    update jig_curation_data
    set affiliations = $2
    where jig_id = $1 and $2 is distinct from affiliations
                "#,
            jig_id.0,
            affiliations,
        )
        .execute(&mut txn)
        .await?;
    }

    // update nullable fields
    if let Some(additional_resources) = additional_resources {
        sqlx::query!(
            //language=SQL
            r#"
        update jig_curation_data
        set additional_resources = $2
        where jig_id = $1 and $2 is distinct from additional_resources
                    "#,
            jig_id.0,
            additional_resources,
        )
        .execute(&mut txn)
        .await?;
    }

    if let Some(curation_status) = curation_status {
        sqlx::query!(
            //language=SQL
            r#"
            update jig_curation_data
            set curation_status = $2
            where jig_id = $1 and $2 is distinct from curation_status
             "#,
            jig_id.0,
            curation_status as i16,
        )
        .execute(&mut txn)
        .await?;
    }

    sqlx::query!(
        r#"
        update jig_curation_data
        set updated_at = now()
        where jig_id = $1
    "#,
        jig_id.0,
    )
    .execute(&mut txn)
    .await?;

    txn.commit().await?;

    Ok(())
}

pub async fn get_curation(pool: &PgPool, jig_id: JigId) -> anyhow::Result<Option<JigCurationData>> {
    let curation = sqlx::query!(
        //language=SQL
        r#"
select jig_id                               as "jig_id!: JigId",
       display_name,
       language,
       goals,
       categories,
       description,
       age_ranges,
       affiliations,
       additional_resources,
       curation_status                          as "curation_status: JigCurationStatus",
       array(
            select row (jcc.id, jcc.jig_id, comment, created_at, author_id)
            from jig_curation_comment  "jcc"
            where jcd.jig_id = jcc.jig_id
        )                                                    as "comments!: Vec<(CommentId, JigId, String, DateTime<Utc>, Uuid)>"
from jig_curation_data "jcd"
where jig_id = $1
"#,
        jig_id.0,
    )
    .fetch_optional(pool)
    .await?
    .map(|row| JigCurationData {
        jig_id: row.jig_id,
        curation_status: row.curation_status,
        fields_done: JigCurationFieldsDone {
            display_name: row.display_name,
            language: row.language,
            goals: row.goals,
            categories: row.categories,
            description: row.description,
            age_ranges: row.age_ranges,
            affiliations: row.affiliations,
            additional_resources: row.additional_resources,
        },
        comments: row.comments.into_iter().map(|(id, jig_id, comment, created_at, author_id)| {
            JigCurationComment {
                id,
                jig_id,
                value: comment,
                created_at,
                author_id
            }}).collect(),
    });

    Ok(curation)
}
