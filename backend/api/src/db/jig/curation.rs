use chrono::{DateTime, Utc};
use shared::domain::jig::{
    curation::{
        CommentId, JigCurationComment, JigCurationCommentResponse, JigCurationData,
        JigCurationFieldsDone, JigCurationStatus,
    },
    report::{JigReportType as RType, ReportId},
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
            order by created_at desc
        )                                                    as "comments!: Vec<(CommentId, JigId, String, DateTime<Utc>, Uuid)>",
        array(
            select row (jr.id, jr.jig_id, report_type, reporter_id, created_at)
            from jig_report "jr"
            where jcd.jig_id = jr.jig_id
            order by created_at desc
        )                                                    as "reports!: Vec<(ReportId, JigId, RType, Uuid, DateTime<Utc>)>"
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
            reports: row.reports.into_iter().map(|(id, jig_id, report_type, created_at, author_id)| {
                JigReport {
                    id,
                    jig_id,
                    value: comment,
                    created_at,
                    author_id
                }}).collect(),
    });

    Ok(curation)
}

pub async fn create_comment(
    pool: &PgPool,
    jig_id: JigId,
    value: String,
    author_id: Uuid,
) -> anyhow::Result<CommentId> {
    // Checks if Audio and Image IDs exists
    sqlx::query!(
        r#"
insert into jig_curation_comment (jig_id, comment, author_id)
values ($1, $2, $3)
returning id as "id!: CommentId"
        "#,
        jig_id.0,
        value,
        author_id
    )
    .fetch_one(pool)
    .await
    .map(|it| it.id)
    .map_err(Into::into)
}

pub async fn get_comment(
    pool: &PgPool,
    jig_id: JigId,
    comment_id: CommentId,
) -> anyhow::Result<Option<JigCurationCommentResponse>> {
    let comment = sqlx::query!(
        //language=SQL
        r#"
select id                                   as "id!: CommentId",
       jig_id                               as "jig_id!: JigId",                      
       comment,
       created_at,
       author_id                            as "author_id!: Uuid",
       (
            select given_name || ' '::text || family_name
            from user_profile
            where user_profile.user_id = author_id
        )                                       as "author_name!"
from jig_curation_comment
where id = $1 and jig_id = $2
"#,
        comment_id.0,
        jig_id.0
    )
    .fetch_optional(pool)
    .await?
    .map(|row| JigCurationCommentResponse {
        id: row.id,
        jig_id: row.jig_id,
        value: row.comment,
        created_at: Some(row.created_at),
        author_id: row.author_id,
        author_name: row.author_name,
    });

    Ok(comment)
}
