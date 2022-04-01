use crate::translate::translate_text;
use anyhow::Context;
use serde_json::value::Value;
use shared::domain::{
    additional_resource::{AdditionalResource, AdditionalResourceId as AddId, ResourceContent},
    category::CategoryId,
    jig::{DraftOrLive, JigId, PrivacyLevel},
    learning_path::{LearningPathData, LearningPathId, LearningPathResponse},
    meta::{AffiliationId, AgeRangeId, ResourceTypeId as TypeId},
    user::UserScope,
};
use sqlx::{types::Json, PgConnection, PgPool};
use std::collections::HashMap;
use tracing::{instrument, Instrument};
use uuid::Uuid;

use crate::error;

pub(crate) mod additional_resource;

pub async fn create(
    pool: &PgPool,
    display_name: &str,
    categories: &[CategoryId],
    age_ranges: &[AgeRangeId],
    affiliations: &[AffiliationId],
    creator_id: Uuid,
    language: &str,
    description: &str,
) -> Result<LearningPathId, CreateLearningPathError> {
    let mut txn = pool.begin().await?;

    let draft_id = create_learning_path_data(
        &mut txn,
        display_name,
        categories,
        age_ranges,
        affiliations,
        language,
        description,
        DraftOrLive::Draft,
    )
    .await?;

    let live_id = create_learning_path_data(
        &mut txn,
        display_name,
        categories,
        age_ranges,
        affiliations,
        language,
        description,
        DraftOrLive::Live,
    )
    .await?;

    let learning_path = sqlx::query!(
        //language=SQL
        r#"insert into learning_path (creator_id, author_id, live_id, draft_id) values ($1, $1, $2, $3) returning id"#,
        creator_id,
        live_id,
        draft_id,
    )
    .fetch_one(&mut txn)
    .await?;

    txn.commit().await?;

    Ok(LearningPathId(learning_path.id))
}

pub async fn create_learning_path_data(
    txn: &mut PgConnection,
    display_name: &str,
    categories: &[CategoryId],
    age_ranges: &[AgeRangeId],
    affiliations: &[AffiliationId],
    language: &str,
    description: &str,
    draft_or_live: DraftOrLive,
) -> Result<Uuid, CreateLearningPathError> {
    log::warn!("description: {}", description);

    let learning_path_data = sqlx::query!(
        // language=SQL
        r#"
insert into learning_path_data
   (display_name, language, description, draft_or_live)
values ($1, $2, $3, $4)
returning id
"#,
        display_name,
        language,
        description,
        draft_or_live as i16,
    )
    .fetch_one(&mut *txn)
    .await?;

    super::recycle_metadata(
        &mut *txn,
        "learning_path_data",
        learning_path_data.id,
        categories,
    )
    .await?;

    super::recycle_metadata(
        &mut *txn,
        "learning_path_data",
        learning_path_data.id,
        age_ranges,
    )
    .await?;

    super::recycle_metadata(
        &mut *txn,
        "learning_path_data",
        learning_path_data.id,
        affiliations,
    )
    .await?;

    Ok(learning_path_data.id)
}

pub enum CreateLearningPathError {
    Sqlx(sqlx::Error),
    InternalServerError(anyhow::Error),
}

impl From<sqlx::Error> for CreateLearningPathError {
    fn from(e: sqlx::Error) -> Self {
        Self::Sqlx(e)
    }
}

impl From<anyhow::Error> for CreateLearningPathError {
    fn from(e: anyhow::Error) -> Self {
        Self::InternalServerError(e)
    }
}

pub async fn get_one(
    pool: &PgPool,
    id: LearningPathId,
    draft_or_live: DraftOrLive,
) -> anyhow::Result<Option<LearningPathResponse>> {
    let res = sqlx::query!( //language=SQL
        r#"
with cte as (
    select id      as "learning_path_id",
           creator_id,
           author_id,
           likes,
           plays,
           case
               when $2 = 0 then learning_path.draft_id
               when $2 = 1 then learning_path.live_id
               end as "draft_or_live_id",
           published_at
    from learning_path
    where id = $1
)
select cte.learning_path_id                                          as "learning_path_id: LearningPathId",
       display_name,
       creator_id,
       author_id,
       (select given_name || ' '::text || family_name
        from user_profile
        where user_profile.user_id = author_id)            as "author_name",
       published_at,
       updated_at,
       privacy_level                                       as "privacy_level!: PrivacyLevel",
       language,
       description,
       translated_description                              as "translated_description!: Json<HashMap<String, String>>",
       likes,
       plays,
       other_keywords,
       translated_keywords,
       array(select row (category_id)
             from learning_path_data_category
             where learning_path_data_id = cte.draft_or_live_id)     as "categories!: Vec<(CategoryId,)>",
       array(select row (affiliation_id)
             from learning_path_data_affiliation
             where learning_path_data_id = cte.draft_or_live_id)     as "affiliations!: Vec<(AffiliationId,)>",
       array(select row (age_range_id)
             from learning_path_data_age_range
             where learning_path_data_id = cte.draft_or_live_id)     as "age_ranges!: Vec<(AgeRangeId,)>",
       array(
             select row (jdar.id, jdar.display_name, resource_type_id, resource_content)
             from learning_path_data_resource "jdar"
             where jdar.learning_path_data_id = cte.draft_or_live_id
       )                                                    as "additional_resource!: Vec<(AddId, String, TypeId, Value)>",
       array(
           select row(jig_id)
           from learning_path_data_jig
           where learning_path_data_jig.jig_id = cte.draft_or_live_id
       )                                                     as "items!: Vec<JigId>"
from learning_path_data
         inner join cte on cte.draft_or_live_id = learning_path_data.id
"#,
        id.0,
        draft_or_live as i16,
    )
        .fetch_optional(pool).await?;

    let learning_path = res.map(|row| LearningPathResponse {
        id: row.learning_path_id,
        published_at: row.published_at,
        creator_id: row.creator_id,
        author_id: row.author_id,
        author_name: row.author_name,
        likes: row.likes,
        plays: row.plays,
        learning_path_data: LearningPathData {
            draft_or_live,
            display_name: row.display_name,
            language: row.language,
            categories: row.categories.into_iter().map(|(it,)| it).collect(),
            last_edited: row.updated_at,
            description: row.description,
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
            privacy_level: row.privacy_level,
            other_keywords: row.other_keywords,
            translated_keywords: row.translated_keywords,
            translated_description: row.translated_description.0,
            items: row.items,
        },
    });

    Ok(learning_path)
}

pub async fn get_by_ids(
    db: &PgPool,
    ids: &[Uuid],
    draft_or_live: DraftOrLive,
) -> sqlx::Result<Vec<LearningPathResponse>> {
    let mut txn = db.begin().await?;

    let learning_path = sqlx::query!(
        //language=SQL
        r#"
select learning_path.id                                       as "id!: LearningPathId",
       creator_id,
       author_id                                as "author_id",
       (select given_name || ' '::text || family_name
        from user_profile
        where user_profile.user_id = author_id) as "author_name",
       live_id                                  as "live_id!",
       draft_id                                 as "draft_id!",
       published_at,
       likes                                    as "likes!",
       plays                                    as "plays!"
from learning_path
         inner join unnest($1::uuid[])
    with ordinality t(id, ord) using (id)
    "#,
        ids,
    )
    .fetch_all(&mut txn)
    .await?;

    let learning_path_data_ids: Vec<Uuid> = match draft_or_live {
        DraftOrLive::Draft => learning_path.iter().map(|it| it.draft_id).collect(),
        DraftOrLive::Live => learning_path.iter().map(|it| it.live_id).collect(),
    };

    let learning_path_data = sqlx::query!(
        //language=SQL
        r#"
select  id,
        display_name                                       as "display_name!",
        updated_at,
        privacy_level                                      as "privacy_level!: PrivacyLevel",
        language                                           as "language!",           
        description                                         as "description!",
        translated_description                              as "translated_description!: Json<HashMap<String, String>>",
        other_keywords                             as "other_keywords!",
        translated_keywords                        as "translated_keywords!",
        array(select row (category_id)
            from learning_path_data_category
            where learning_path_data_id = learning_path_data.id)     as "categories!: Vec<(CategoryId,)>",
        array(select row (affiliation_id)
            from learning_path_data_affiliation
            where learning_path_data_id = learning_path_data.id)     as "affiliations!: Vec<(AffiliationId,)>",
        array(select row (age_range_id)
            from learning_path_data_age_range
            where learning_path_data_id = learning_path_data.id)     as "age_ranges!: Vec<(AgeRangeId,)>",
        array(
            select row (jdar.id, jdar.display_name, resource_type_id, resource_content)
            from learning_path_data_resource "jdar"
            where jdar.learning_path_data_id = learning_path_data.id
        )                                                    as "additional_resource!: Vec<(AddId, String, TypeId, Value)>",
        array(
            select row(jig_id)
            from learning_path_data_jig
            where learning_path_data_jig.learning_path_data_id = learning_path_data.id
        )                                                     as "items!: Vec<JigId>"
from learning_path_data
         inner join unnest($1::uuid[])
    with ordinality t(id, ord) using (id)
"#,
        &learning_path_data_ids
    )
        .fetch_all(&mut txn)
        .await?;

    let v = learning_path
        .into_iter()
        .zip(learning_path_data.into_iter())
        .map(
            |(learning_path_row, learning_path_data_row)| LearningPathResponse {
                id: learning_path_row.id,
                published_at: learning_path_row.published_at,
                creator_id: learning_path_row.creator_id,
                author_id: learning_path_row.author_id,
                author_name: learning_path_row.author_name,
                likes: learning_path_row.likes,
                plays: learning_path_row.plays,
                learning_path_data: LearningPathData {
                    draft_or_live,
                    display_name: learning_path_data_row.display_name,
                    language: learning_path_data_row.language,
                    categories: learning_path_data_row
                        .categories
                        .into_iter()
                        .map(|(it,)| it)
                        .collect(),
                    last_edited: learning_path_data_row.updated_at,
                    description: learning_path_data_row.description,
                    age_ranges: learning_path_data_row
                        .age_ranges
                        .into_iter()
                        .map(|(it,)| it)
                        .collect(),
                    affiliations: learning_path_data_row
                        .affiliations
                        .into_iter()
                        .map(|(it,)| it)
                        .collect(),
                    additional_resources: learning_path_data_row
                        .additional_resource
                        .into_iter()
                        .map(|(id, display_name, resource_type_id, resource_content)| {
                            AdditionalResource {
                                id,
                                display_name,
                                resource_type_id,
                                resource_content: serde_json::from_value::<ResourceContent>(
                                    resource_content,
                                )
                                .unwrap(),
                            }
                        })
                        .collect(),
                    privacy_level: learning_path_data_row.privacy_level,
                    other_keywords: learning_path_data_row.other_keywords,
                    translated_keywords: learning_path_data_row.translated_keywords,
                    translated_description: learning_path_data_row.translated_description.0,
                    items: learning_path_data_row.items,
                },
            },
        )
        .collect();

    txn.rollback().await?;

    Ok(v)
}

#[instrument(skip(db))]
pub async fn browse(
    db: &sqlx::Pool<sqlx::Postgres>,
    author_id: Option<Uuid>,
    draft_or_live: Option<DraftOrLive>,
    privacy_level: Vec<PrivacyLevel>,
    page: i32,
    page_limit: u32,
    resource_types: Vec<Uuid>,
) -> sqlx::Result<Vec<LearningPathResponse>> {
    let mut txn = db.begin().await?;

    let privacy_level: Vec<i16> = privacy_level.iter().map(|x| *x as i16).collect();

    //TODO: purge junk jig data from with draft_or_live is NULL
    let learning_path_data = sqlx::query!(
    //language=SQL
    r#"
with cte as (
    select array(select jd.id as "id!"
    from learning_path_data "jd"
          left join learning_path on (draft_id = jd.id or (live_id = jd.id and jd.last_synced_at is not null))
          left join learning_path_data_resource "resource" on jd.id = resource.learning_path_data_id
    where (author_id = $1 or $1 is null)
        and (jd.draft_or_live = $2 or $2 is null)
        and (jd.privacy_level = any($3) or $3 = array[]::smallint[])
        and (resource.resource_type_id = any($4) or $4 = array[]::uuid[])
    order by coalesce(updated_at, created_at) desc) as id
),
cte1 as (
    select * from unnest((select distinct id from cte)) with ordinality t(id
   , ord) order by ord
)
select learning_path.id                                                         as "learning_path_id: LearningPathId",
    privacy_level                                                               as "privacy_level: PrivacyLevel",
    creator_id,
    author_id,
    (select given_name || ' '::text || family_name
     from user_profile
     where user_profile.user_id = author_id)                                     as "author_name",
    published_at,
    likes,
    plays,
    display_name                                                                  as "display_name!",
    updated_at,
    language                                                                      as "language!",
    description                                                                   as "description!",
    translated_description                                                        as "translated_description!: Json<HashMap<String,String>>",
    draft_or_live                                                                 as "draft_or_live!: DraftOrLive",
    other_keywords                                                                as "other_keywords!",
    translated_keywords                                                           as "translated_keywords!",
    array(select row (category_id)
            from learning_path_data_category
            where learning_path_data_id = learning_path_data.id)     as "categories!: Vec<(CategoryId,)>",
    array(select row (affiliation_id)
            from learning_path_data_affiliation
            where learning_path_data_id = learning_path_data.id)     as "affiliations!: Vec<(AffiliationId,)>",
    array(select row (age_range_id)
            from learning_path_data_age_range
            where learning_path_data_id = learning_path_data.id)     as "age_ranges!: Vec<(AgeRangeId,)>",
    array(
                select row (jdar.id, jdar.display_name, resource_type_id, resource_content)
                from learning_path_data_resource "jdar"
                where jdar.learning_path_data_id = learning_path_data.id
            )                                               as "additional_resource!: Vec<(AddId, String, TypeId, Value)>",
    array(
        select row(jig_id)
        from learning_path_data_jig
        where learning_path_data_jig.learning_path_data_id = learning_path_data.id
    )                                                     as "items!: Vec<JigId>"
from cte1
left join learning_path_data on cte1.id = learning_path_data.id
left join learning_path on (learning_path_data.id = learning_path.draft_id or (learning_path_data.id = learning_path.live_id and last_synced_at is not null))
where cte1.ord > (1 * $5 * $6)
limit $6
"#,
    author_id,
    draft_or_live.map(|it| it as i16),
    &privacy_level[..],
    &resource_types[..],
    page,
    page_limit as i32,
)
    .fetch_all(&mut txn)
    .instrument(tracing::info_span!("query learning_path_data"))
    .await?;

    let v: Vec<_> = learning_path_data
        .into_iter()
        .map(|learning_path_data_row| LearningPathResponse {
            id: learning_path_data_row.learning_path_id,
            published_at: learning_path_data_row.published_at,
            creator_id: learning_path_data_row.creator_id,
            author_id: learning_path_data_row.author_id,
            author_name: learning_path_data_row.author_name,
            likes: learning_path_data_row.likes,
            plays: learning_path_data_row.plays,
            learning_path_data: LearningPathData {
                draft_or_live: learning_path_data_row.draft_or_live,
                display_name: learning_path_data_row.display_name,
                language: learning_path_data_row.language,
                categories: learning_path_data_row
                    .categories
                    .into_iter()
                    .map(|(it,)| it)
                    .collect(),
                last_edited: learning_path_data_row.updated_at,
                description: learning_path_data_row.description,
                age_ranges: learning_path_data_row
                    .age_ranges
                    .into_iter()
                    .map(|(it,)| it)
                    .collect(),
                affiliations: learning_path_data_row
                    .affiliations
                    .into_iter()
                    .map(|(it,)| it)
                    .collect(),
                additional_resources: learning_path_data_row
                    .additional_resource
                    .into_iter()
                    .map(|(id, display_name, resource_type_id, resource_content)| {
                        AdditionalResource {
                            id,
                            display_name,
                            resource_type_id,
                            resource_content: serde_json::from_value::<ResourceContent>(
                                resource_content,
                            )
                            .unwrap(),
                        }
                    })
                    .collect(),
                privacy_level: learning_path_data_row.privacy_level,
                other_keywords: learning_path_data_row.other_keywords,
                translated_keywords: learning_path_data_row.translated_keywords,
                translated_description: learning_path_data_row.translated_description.0,
                items: learning_path_data_row.items,
            },
        })
        .collect();

    txn.rollback().await?;

    Ok(v)
}

pub async fn update_draft(
    pool: &PgPool,
    api_key: &Option<String>,
    id: LearningPathId,
    display_name: Option<&str>,
    categories: Option<&[CategoryId]>,
    age_ranges: Option<&[AgeRangeId]>,
    affiliations: Option<&[AffiliationId]>,
    language: Option<&str>,
    description: Option<&str>,
    privacy_level: Option<PrivacyLevel>,
    other_keywords: Option<String>,
    jig_ids: Option<&[JigId]>,
) -> Result<(), error::UpdateWithMetadata> {
    let mut txn = pool.begin().await?;

    let draft_id = sqlx::query!(
        //language=SQL
        r#"
select draft_id from learning_path join learning_path_data on learning_path.draft_id = learning_path_data.id where learning_path.id = $1 for update
"#,
        id.0
    )
    .fetch_optional(&mut txn)
    .await?
    .ok_or(error::UpdateWithMetadata::ResourceNotFound)?
    .draft_id;

    if let Some(privacy_level) = privacy_level {
        sqlx::query!(
            //language=SQL
            r#"
update learning_path_data
set privacy_level = coalesce($2, privacy_level),
    updated_at = now()
where id = $1
  and $2 is distinct from privacy_level
    "#,
            draft_id,
            privacy_level as i16,
        )
        .execute(&mut txn)
        .await?;
    }

    if let Some(description) = description {
        sqlx::query!(
            r#"
update learning_path_data
set description = $2,
    updated_at = now()
where id = $1 and $2 is distinct from description"#,
            draft_id,
            description,
        )
        .execute(&mut txn)
        .await?;
    }

    if let Some(other_keywords) = other_keywords {
        let translate_text = match &api_key {
            Some(key) => translate_text(&other_keywords, "he", "en", key)
                .await
                .context("could not translate text")?,
            None => None,
        };

        sqlx::query!(
            r#"
update learning_path_data
set other_keywords = $2,
    translated_keywords = (case when ($3::text is not null) then $3::text else (translated_keywords) end),
    updated_at = now()
where id = $1 and $2 is distinct from other_keywords"#,
            draft_id,
            other_keywords,
            translate_text
        )
        .execute(&mut *txn)
        .await?;
    }

    // update trivial, not null fields
    sqlx::query!(
        //language=SQL
        r#"
update learning_path_data
set display_name     = coalesce($2, display_name),
    language         = coalesce($3, language),
    updated_at = now()
where id = $1
  and (($2::text is not null and $2 is distinct from display_name) or
       ($3::text is not null and $3 is distinct from language))
"#,
        draft_id,
        display_name,
        language,
    )
    .execute(&mut txn)
    .await?;

    if let Some(categories) = categories {
        super::recycle_metadata(&mut txn, "learning_path_data", draft_id, categories)
            .await
            .map_err(super::meta::handle_metadata_err)?;
    }

    if let Some(affiliations) = affiliations {
        super::recycle_metadata(&mut txn, "learning_path_data", draft_id, affiliations)
            .await
            .map_err(super::meta::handle_metadata_err)?;
    }

    if let Some(age_ranges) = age_ranges {
        super::recycle_metadata(&mut txn, "learning_path_data", draft_id, age_ranges)
            .await
            .map_err(super::meta::handle_metadata_err)?;
    }

    if let Some(jig) = jig_ids {
        super::recycle_metadata(&mut txn, "learning_path_data", draft_id, jig)
            .await
            .map_err(super::meta::handle_metadata_err)?;
    }

    txn.commit().await?;

    Ok(())
}

pub async fn delete(pool: &PgPool, id: LearningPathId) -> Result<(), error::Delete> {
    let mut txn = pool.begin().await?;

    let (draft_id, live_id) = get_draft_and_live_ids(&mut txn, id)
        .await
        .ok_or(error::Delete::ResourceNotFound)?;

    sqlx::query!(
        //language=SQL
        r#"
with del_data as (
    delete from learning_path_data
        where id is not distinct from $1 or id is not distinct from $2)
delete
from learning_path
where id is not distinct from $3

"#,
        draft_id,
        live_id,
        id.0,
    )
    .execute(&mut txn)
    .await?;

    txn.commit().await?;
    Ok(())
}

// `None` here means do not filter.
#[instrument(skip(db))]
pub async fn filtered_count(
    db: &PgPool,
    privacy_level: Vec<PrivacyLevel>,
    author_id: Option<Uuid>,
    draft_or_live: Option<DraftOrLive>,
    resource_types: Vec<Uuid>,
) -> sqlx::Result<(u64, u64)> {
    let privacy_level: Vec<i16> = privacy_level.iter().map(|x| *x as i16).collect();

    let learning_path_data = sqlx::query!(
        //language=SQL
        r#"
select count(distinct learning_path_data.id) as "count!: i64"
from learning_path_data
left join learning_path on (draft_id = learning_path_data.id or (live_id = learning_path_data.id and last_synced_at is not null))
left join learning_path_data_resource "resource" on learning_path_data.id = resource.learning_path_data_id
where (author_id = $1 or $1 is null)
    and (learning_path_data.draft_or_live = $2 or $2 is null)
    and (learning_path_data.privacy_level = any($3) or $3 = array[]::smallint[])
    and (resource.resource_type_id = any($4) or $4 = array[]::uuid[])
"#,
        author_id,
        draft_or_live.map(|it| it as i16),
        &privacy_level[..],
        &resource_types[..]
    )
    .fetch_one(db)
    .await?;

    let learning_path = sqlx::query!(
        //language=SQL
        r#"
select count(distinct learning_path.id) as "count!: i64"
from learning_path
left join learning_path_data on (draft_id = learning_path_data.id or (live_id = learning_path_data.id and last_synced_at is not null))
left join learning_path_data_resource "resource" on learning_path_data.id = resource.learning_path_data_id
where (author_id = $1 or $1 is null)
    and (learning_path_data.draft_or_live = $2 or $2 is null)
    and (learning_path_data.privacy_level = any($3) or $3 = array[]::smallint[])
    and (resource.resource_type_id = any($4) or $4 = array[]::uuid[])
"#,
        author_id,
        draft_or_live.map(|it| it as i16),
        &privacy_level[..],
        &resource_types[..]
    )
    .fetch_one(db)
    .await?;

    Ok((learning_path.count as u64, learning_path_data.count as u64))
}

pub async fn get_draft_and_live_ids(
    txn: &mut PgConnection,
    learning_path_id: LearningPathId,
) -> Option<(Uuid, Uuid)> {
    sqlx::query!(
        //language=SQL
        r#"
select draft_id, live_id from learning_path where id = $1
"#,
        learning_path_id.0
    )
    .fetch_optional(&mut *txn)
    .await
    .ok()?
    .map(|it| (it.draft_id, it.live_id))
}

/// Clones a copy of the jig data and modules, preserving the module's stable IDs
pub async fn clone_data(
    txn: &mut PgConnection,
    from_data_id: &Uuid,
    draft_or_live: DraftOrLive,
) -> Result<Uuid, error::CloneDraft> {
    println!("here in clone");
    let new_id = sqlx::query!(
        //language=SQL
        r#"
insert into learning_path_data
(display_name, created_at, updated_at, language, last_synced_at, description, privacy_level, other_keywords, translated_keywords, translated_description)
select display_name,
       created_at,
       updated_at,
       language,
       last_synced_at,
       description,
       privacy_level,
       other_keywords,
       translated_keywords,
       translated_description::jsonb
from learning_path_data
where id = $1
returning id
        "#,
        from_data_id,
    )
    .fetch_one(&mut *txn)
    .await?
    .id;

    update_draft_or_live(txn, new_id, draft_or_live).await?;

    sqlx::query!(
        //language=SQL
        r#"
insert into learning_path_data_resource(learning_path_data_id, resource_type_id, display_name, resource_content)
select $2, resource_type_id, display_name, resource_content
from learning_path_data_resource
where learning_path_data_id = $1
        "#,
        from_data_id,
        new_id,
    )
    .execute(&mut *txn)
    .await?;

    sqlx::query!(
        //language=SQL
        r#"
insert into learning_path_data_affiliation(learning_path_data_id, affiliation_id)
select $2, affiliation_id
from learning_path_data_affiliation
where learning_path_data_id = $1
        "#,
        from_data_id,
        new_id,
    )
    .execute(&mut *txn)
    .await?;

    sqlx::query!(
        //language=SQL
        r#"
insert into learning_path_data_age_range(learning_path_data_id, age_range_id)
select $2, age_range_id
from learning_path_data_age_range
where learning_path_data_id = $1
        "#,
        from_data_id,
        new_id,
    )
    .execute(&mut *txn)
    .await?;

    sqlx::query!(
        //language=SQL
        r#"
insert into learning_path_data_category(learning_path_data_id, category_id)
select $2, category_id
from learning_path_data_category
where learning_path_data_id = $1
        "#,
        from_data_id,
        new_id,
    )
    .execute(&mut *txn)
    .await?;

    sqlx::query!(
        //language=SQL
        r#"
insert into learning_path_data_jig(learning_path_data_id, jig_id)
select $2, jig_id
from learning_path_data_jig
where learning_path_data_id = $1
        "#,
        from_data_id,
        new_id,
    )
    .execute(&mut *txn)
    .await?;

    Ok(new_id)
}

pub async fn is_admin(db: &PgPool, user_id: Uuid) -> Result<bool, error::Auth> {
    let authed = sqlx::query!(
        r#"
select exists(select 1 from user_scope where user_id = $1 and scope = any($2)) as "authed!"
"#,
        user_id,
        &[UserScope::Admin as i16, UserScope::AdminJig as i16][..],
    )
    .fetch_one(db)
    .await?
    .authed;

    if !authed {
        return Ok(false);
    }

    Ok(true)
}

pub async fn authz(
    db: &PgPool,
    user_id: Uuid,
    learning_path_id: Option<LearningPathId>,
) -> Result<(), error::Auth> {
    let authed = match learning_path_id {
        None => {
            sqlx::query!(
                r#"
select exists(select 1 from user_scope where user_id = $1 and scope = any($2)) as "authed!"
"#,
                user_id,
                &[
                    UserScope::Admin as i16,
                    UserScope::AdminJig as i16,
                    UserScope::ManageSelfJig as i16,
                ][..],
            )
            .fetch_one(db)
            .await?
            .authed
        }
        Some(id) => {
            sqlx::query!(
                //language=SQL
                r#"
select exists (
    select 1 from user_scope where user_id = $1 and scope = any($2)
) or (
    exists (select 1 from user_scope where user_id = $1 and scope = $3) and
    not exists (select 1 from learning_path where learning_path.id = $4 and learning_path.author_id <> $1)
) as "authed!"
"#,
                user_id,
                &[UserScope::Admin as i16, UserScope::AdminJig as i16,][..],
                UserScope::ManageSelfJig as i16,
                id.0
            )
            .fetch_one(db)
            .await?
            .authed
        }
    };

    if !authed {
        return Err(error::Auth::Forbidden);
    }

    Ok(())
}

async fn update_draft_or_live(
    conn: &mut PgConnection,
    learning_path_data_id: Uuid,
    draft_or_live: DraftOrLive,
) -> sqlx::Result<()> {
    sqlx::query!(
        //language=SQL
        r#"
update learning_path_data
set draft_or_live = $2
where id = $1
            "#,
        learning_path_data_id,
        draft_or_live as i16
    )
    .execute(&mut *conn)
    .await?;

    Ok(())
}
