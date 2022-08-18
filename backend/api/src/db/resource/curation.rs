use shared::domain::{
    resource::{
        curation::{
            CommentId as CommId, ResourceCurationComment, ResourceCurationCommentResponse,
            ResourceCurationData, ResourceCurationFieldsDone, ResourceCurationStatus,
        },
        report::ResourceReport,
        ResourceId,
    },
    user::UserId,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error;

pub async fn update(
    pool: &PgPool,
    resource_id: ResourceId,
    display_name: Option<bool>,
    categories: Option<bool>,
    age_ranges: Option<bool>,
    affiliations: Option<bool>,
    language: Option<bool>,
    description: Option<bool>,
    additional_resources: Option<bool>,
    curation_status: Option<ResourceCurationStatus>,
) -> anyhow::Result<(), error::Auth> {
    let mut txn = pool.begin().await?;

    if let Some(display_name) = display_name {
        sqlx::query!(
            //language=SQL
            r#"
update resource_curation_data
set display_name = $2
where resource_id = $1 and $2 is distinct from display_name
            "#,
            resource_id.0,
            display_name,
        )
        .execute(&mut txn)
        .await?;
    }

    if let Some(language) = language {
        sqlx::query!(
            //language=SQL
            r#"
update resource_curation_data
set language = $2
where resource_id = $1 and $2 is distinct from language
            "#,
            resource_id.0,
            language,
        )
        .execute(&mut txn)
        .await?;
    }

    if let Some(categories) = categories {
        sqlx::query!(
            //language=SQL
            r#"
update resource_curation_data
set categories = $2
where resource_id = $1 and $2 is distinct from categories
            "#,
            resource_id.0,
            categories,
        )
        .execute(&mut txn)
        .await?;
    }

    if let Some(description) = description {
        sqlx::query!(
            //language=SQL
            r#"
    update resource_curation_data
    set description = $2
    where resource_id = $1 and $2 is distinct from description
                "#,
            resource_id.0,
            description,
        )
        .execute(&mut txn)
        .await?;
    }

    if let Some(age_ranges) = age_ranges {
        sqlx::query!(
            //language=SQL
            r#"
    update resource_curation_data
    set age_ranges = $2
    where resource_id = $1 and $2 is distinct from age_ranges
                "#,
            resource_id.0,
            age_ranges,
        )
        .execute(&mut txn)
        .await?;
    }

    if let Some(affiliations) = affiliations {
        sqlx::query!(
            //language=SQL
            r#"
    update resource_curation_data
    set affiliations = $2
    where resource_id = $1 and $2 is distinct from affiliations
                "#,
            resource_id.0,
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
        update resource_curation_data
        set additional_resources = $2
        where resource_id = $1 and $2 is distinct from additional_resources
                    "#,
            resource_id.0,
            additional_resources,
        )
        .execute(&mut txn)
        .await?;
    }

    if let Some(curation_status) = curation_status {
        sqlx::query!(
            //language=SQL
            r#"
            update resource_curation_data
            set curation_status = $2
            where resource_id = $1 and $2 is distinct from curation_status
             "#,
            resource_id.0,
            curation_status as i16,
        )
        .execute(&mut txn)
        .await?;
    }

    sqlx::query!(
        r#"
        update resource_curation_data
        set updated_at = now()
        where resource_id = $1
    "#,
        resource_id.0,
    )
    .execute(&mut txn)
    .await?;

    txn.commit().await?;

    Ok(())
}

pub async fn get_curation(
    pool: &PgPool,
    resource_id: ResourceId,
) -> anyhow::Result<Option<ResourceCurationData>> {
    let curation = sqlx::query!(
        //language=SQL
        r#"
select resource_id                               as "resource_id!: ResourceId",
       display_name,
       language,
       categories,
       description,
       age_ranges,
       affiliations,
       additional_resources,
       curation_status                          as "curation_status!: ResourceCurationStatus",
       array(
            select row (rcc.id, rcc.resource_id, comment, created_at, author_id)
            from resource_curation_comment  "rcc"
            where rcd.resource_id = rcc.resource_id
            order by created_at desc
       )                                         as "comments!: Vec<(ResourceCurationComment)>",
       array(
           select row (rr.id, rr.resource_id, report_type, reporter_id, created_at,      
                        (
                        select given_name || ' '::text || family_name
                        from user_profile
                        where user_profile.user_id = reporter_id
                        ),
                        (
                            select email::text
                            from user_email
                            where user_email.user_id = reporter_id
                        )                                                                       
            )
           from resource_report "rr"
           where rcd.resource_id = rr.resource_id
           order by created_at desc
       )                                                    as "reports!: Vec<(ResourceReport)>"
from resource_curation_data "rcd"
where resource_id = $1
"#,
        resource_id.0,
    )
    .fetch_optional(pool)
    .await?
    .map(|row| ResourceCurationData {
        resource_id: row.resource_id,
        curation_status: row.curation_status,
        fields_done: ResourceCurationFieldsDone {
            display_name: row.display_name,
            language: row.language,
            categories: row.categories,
            description: row.description,
            age_ranges: row.age_ranges,
            affiliations: row.affiliations,
            additional_resources: row.additional_resources,
        },
        comments: row.comments.into_iter().map(|it| it).collect(),
        reports: row.reports.into_iter().map(|it| it).collect(),
    });

    Ok(curation)
}

pub async fn create_comment(
    pool: &PgPool,
    resource_id: ResourceId,
    value: String,
    author_id: Uuid,
) -> anyhow::Result<CommId> {
    // Checks if Audio and Image IDs exists
    sqlx::query!(
        r#"
insert into resource_curation_comment (resource_id, comment, author_id)
values ($1, $2, $3)
returning id as "id!: CommId"
        "#,
        resource_id.0,
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
    resource_id: ResourceId,
    comment_id: CommId,
) -> anyhow::Result<Option<ResourceCurationCommentResponse>> {
    let comment = sqlx::query!(
        //language=SQL
        r#"
select id                                   as "id!: CommId",
       resource_id                          as "resource_id!: ResourceId",                      
       comment,
       created_at,
       author_id                            as "author_id!: UserId",
       (
            select given_name || ' '::text || family_name
            from user_profile
            where user_profile.user_id = author_id
        )                                       as "author_name!"
from resource_curation_comment
where id = $1 and resource_id = $2
"#,
        comment_id.0,
        resource_id.0
    )
    .fetch_optional(pool)
    .await?
    .map(|row| ResourceCurationCommentResponse {
        id: row.id,
        resource_id: row.resource_id,
        value: row.comment,
        created_at: Some(row.created_at),
        author_id: row.author_id,
        author_name: row.author_name,
    });

    Ok(comment)
}
