use crate::translate::translate_text;
use anyhow::Context;
use serde_json::value::Value;
use shared::domain::{
    additional_resource::{AdditionalResource, AdditionalResourceId as AddId, ResourceContent},
    asset::{DraftOrLive, OrderBy, PrivacyLevel},
    category::CategoryId,
    meta::{AffiliationId, AgeRangeId, ResourceTypeId as TypeId},
    module::{LiteModule, ModuleId, ModuleKind},
    resource::{ResourceAdminData, ResourceData, ResourceId, ResourceRating, ResourceResponse},
    user::{UserId, UserScope},
};
use sqlx::{types::Json, PgConnection, PgPool};
use std::collections::HashMap;
use tracing::{instrument, Instrument};
use uuid::Uuid;

use crate::error;

pub(crate) mod additional_resource;
pub(crate) mod curation;
pub(crate) mod module;
pub(crate) mod report;

pub async fn create(
    pool: &PgPool,
    display_name: &str,
    categories: &[CategoryId],
    age_ranges: &[AgeRangeId],
    affiliations: &[AffiliationId],
    creator_id: UserId,
    language: &str,
    description: &str,
) -> Result<ResourceId, CreateResourceError> {
    let mut txn = pool.begin().await?;

    let draft_id = create_resource_data(
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

    let live_id = create_resource_data(
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

    let resource = sqlx::query!(
        //language=SQL
        r#"insert into resource (creator_id, author_id, live_id, draft_id) values ($1, $1, $2, $3) returning id"#,
        creator_id.0,
        live_id,
        draft_id,
    )
    .fetch_one(&mut txn)
    .await?;

    txn.commit().await?;

    Ok(ResourceId(resource.id))
}

pub async fn create_resource_data(
    txn: &mut PgConnection, // FIXME does this work?
    display_name: &str,
    categories: &[CategoryId],
    age_ranges: &[AgeRangeId],
    affiliations: &[AffiliationId],
    language: &str,
    description: &str,
    draft_or_live: DraftOrLive,
) -> Result<Uuid, CreateResourceError> {
    let resource_data = sqlx::query!(
        // language=SQL
        r#"
insert into resource_data
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

    super::recycle_metadata(&mut *txn, "resource_data", resource_data.id, categories).await?;
    super::recycle_metadata(&mut *txn, "resource_data", resource_data.id, age_ranges).await?;
    super::recycle_metadata(&mut *txn, "resource_data", resource_data.id, affiliations).await?;

    Ok(resource_data.id)
}

/// Handle errors for creating a module when posting a Resource
/// This is here because the scope is limited to the above function
pub enum CreateResourceError {
    Sqlx(sqlx::Error),
    DefaultModules(serde_json::Error),
    InternalServerError(anyhow::Error),
}

impl From<sqlx::Error> for CreateResourceError {
    fn from(e: sqlx::Error) -> Self {
        Self::Sqlx(e)
    }
}

impl From<anyhow::Error> for CreateResourceError {
    fn from(e: anyhow::Error) -> Self {
        Self::InternalServerError(e)
    }
}

impl From<serde_json::Error> for CreateResourceError {
    fn from(e: serde_json::Error) -> Self {
        Self::DefaultModules(e)
    }
}

#[instrument(skip(pool))]
pub async fn get_one(
    pool: &PgPool,
    id: ResourceId,
    draft_or_live: DraftOrLive,
) -> anyhow::Result<Option<ResourceResponse>> {
    let res = sqlx::query!( //language=SQL
        r#"
with cte as (
    select id      as "resource_id",
           creator_id,
           author_id,
           likes,
           views,
           live_up_to_date,
           case
               when $2 = 0 then resource.draft_id
               when $2 = 1 then resource.live_id
               end as "draft_or_live_id",
           published_at,
           rating,
           blocked,
           curated
    from resource
    left join resource_admin_data "admin" on admin.resource_id = resource.id
    where id = $1
)
select cte.resource_id                                          as "resource_id: ResourceId",
        display_name,
        creator_id                                          as "creator_id: UserId",
        author_id                                           as "author_id: UserId",
        (select given_name || ' '::text || family_name
        from user_profile
        where user_profile.user_id = author_id)            as "author_name",
        created_at,
        updated_at,
        published_at,
        privacy_level                                       as "privacy_level!: PrivacyLevel",
        language,
        description,
        translated_description                              as "translated_description!: Json<HashMap<String, String>>",
        likes,
        views,
        live_up_to_date,
        locked,
        other_keywords,
        translated_keywords,
        rating                                               as "rating?: ResourceRating",
        blocked                                              as "blocked",
        curated,
        (
                select row(resource_data_module.id, kind, is_complete)
                from resource_data_module
                where resource_data_id = resource_data.id
        )                                               as "cover?: (ModuleId, ModuleKind, bool)",
        array(select row (category_id)
                from resource_data_category
                where resource_data_id = cte.draft_or_live_id)     as "categories!: Vec<(CategoryId,)>",
        array(select row (affiliation_id)
                from resource_data_affiliation
                where resource_data_id = cte.draft_or_live_id)     as "affiliations!: Vec<(AffiliationId,)>",
        array(select row (age_range_id)
                from resource_data_age_range
                where resource_data_id = cte.draft_or_live_id)     as "age_ranges!: Vec<(AgeRangeId,)>",
        array(
                select row (rdr.id, rdr.display_name, resource_type_id, resource_content)
                from resource_data_resource "rdr"
                where rdr.resource_data_id = cte.draft_or_live_id
    )                                                    as "additional_resource!: Vec<(AddId, String, TypeId, Value)>"
from resource_data
         inner join cte on cte.draft_or_live_id = resource_data.id
"#,
        id.0,
        draft_or_live as i16,
    )
        .fetch_optional(pool).await?;

    let resource = res.map(|row| ResourceResponse {
        id: row.resource_id,
        published_at: row.published_at,
        creator_id: row.creator_id,
        author_id: row.author_id,
        author_name: row.author_name,
        likes: row.likes,
        views: row.views,
        live_up_to_date: row.live_up_to_date,
        resource_data: ResourceData {
            created_at: row.created_at,
            draft_or_live,
            display_name: row.display_name,
            language: row.language,
            cover: row.cover.map(|(id, kind, is_complete)| LiteModule {
                id,
                kind,
                is_complete,
            }),
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
            locked: row.locked,
            other_keywords: row.other_keywords,
            translated_keywords: row.translated_keywords,
            translated_description: row.translated_description.0,
        },
        admin_data: ResourceAdminData {
            rating: row.rating,
            blocked: row.blocked,
            curated: row.curated,
        },
    });

    Ok(resource)
}

#[instrument(skip(db))]
pub async fn get_by_ids(
    db: &PgPool,
    ids: &[Uuid],
    draft_or_live: DraftOrLive,
) -> sqlx::Result<Vec<ResourceResponse>> {
    let mut txn = db.begin().await?;

    let resource = sqlx::query!(
        //language=SQL
        r#"
select resource.id                                       as "id!: ResourceId",
       creator_id                               as "creator_id: UserId",
       author_id                                as "author_id: UserId",
       (select given_name || ' '::text || family_name
        from user_profile
        where user_profile.user_id = author_id) as "author_name",
       live_id                                  as "live_id!",
       draft_id                                 as "draft_id!",
       published_at,
       likes                                     as "likes!",
       views                                    as "views!",
       live_up_to_date                          as "live_up_to_date!",
       rating                                   as "rating?: ResourceRating",
       blocked                                  as "blocked!",
       curated                                  as "curated!"
from resource
inner join unnest($1::uuid[])
    with ordinality t(id, ord) using (id)
inner join resource_admin_data "admin" on admin.resource_id = resource.id
order by ord asc
    "#,
        ids,
    )
    .fetch_all(&mut txn)
    .instrument(tracing::info_span!("query resources"))
    .await?;

    let resource_data_ids: Vec<Uuid> = match draft_or_live {
        DraftOrLive::Draft => resource.iter().map(|it| it.draft_id).collect(),
        DraftOrLive::Live => resource.iter().map(|it| it.live_id).collect(),
    };

    let resource_data = sqlx::query!(
        //language=SQL
        r#"
select id,
       display_name                                                                  as "display_name!",
       created_at                                                                    as "created_at!",
       updated_at,
       language                                                                      as "language!",
       description                                                                   as "description!",
       translated_description                                                        as "translated_description!: Json<HashMap<String,String>>",
       (
                select row (resource_data_module.id, kind, is_complete)
                from resource_data_module
                where resource_data_id = resource_data.id
       )                                                  as "cover?: (ModuleId, ModuleKind, bool)",
       array(select row (category_id)
             from resource_data_category
             where resource_data_id = resource_data.id)     as "categories!: Vec<(CategoryId,)>",
       array(select row (affiliation_id)
             from resource_data_affiliation
             where resource_data_id = resource_data.id)     as "affiliations!: Vec<(AffiliationId,)>",
       array(select row (age_range_id)
             from resource_data_age_range
             where resource_data_id = resource_data.id)     as "age_ranges!: Vec<(AgeRangeId,)>",
       array(
                select row (rdr.id, rdr.display_name, resource_type_id, resource_content)
                from resource_data_resource "rdr"
                where rdr.resource_data_id = resource_data.id
            )                                               as "additional_resource!: Vec<(AddId, String, TypeId, Value)>",
       privacy_level                              as "privacy_level!: PrivacyLevel",
       locked                                     as "locked!",
       other_keywords                             as "other_keywords!",
       translated_keywords                        as "translated_keywords!"
from resource_data
inner join unnest($1::uuid[])
    with ordinality t(id, ord) using (id)
order by ord asc
"#,
        &resource_data_ids
    )
        .fetch_all(&mut txn)
        .instrument(tracing::info_span!("query resource_data"))
        .await?;

    let v = resource
        .into_iter()
        .zip(resource_data.into_iter())
        .map(|(resource_row, resource_data_row)| ResourceResponse {
            id: resource_row.id,
            published_at: resource_row.published_at,
            creator_id: resource_row.creator_id,
            author_id: resource_row.author_id,
            author_name: resource_row.author_name,
            likes: resource_row.likes,
            views: resource_row.views,
            live_up_to_date: resource_row.live_up_to_date,
            resource_data: ResourceData {
                created_at: resource_data_row.created_at,
                draft_or_live,
                display_name: resource_data_row.display_name,
                language: resource_data_row.language,
                cover: resource_data_row
                    .cover
                    .map(|(id, kind, is_complete)| LiteModule {
                        id,
                        kind,
                        is_complete,
                    }),
                categories: resource_data_row
                    .categories
                    .into_iter()
                    .map(|(it,)| it)
                    .collect(),
                last_edited: resource_data_row.updated_at,
                description: resource_data_row.description,
                age_ranges: resource_data_row
                    .age_ranges
                    .into_iter()
                    .map(|(it,)| it)
                    .collect(),
                affiliations: resource_data_row
                    .affiliations
                    .into_iter()
                    .map(|(it,)| it)
                    .collect(),
                additional_resources: resource_data_row
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
                privacy_level: resource_data_row.privacy_level,
                locked: resource_data_row.locked,
                other_keywords: resource_data_row.other_keywords,
                translated_keywords: resource_data_row.translated_keywords,
                translated_description: resource_data_row.translated_description.0,
            },
            admin_data: ResourceAdminData {
                rating: resource_row.rating,
                blocked: resource_row.blocked,
                curated: resource_row.curated,
            },
        })
        .collect();

    txn.rollback().await?;

    Ok(v)
}

#[instrument(skip(db))]
pub async fn browse(
    db: &sqlx::Pool<sqlx::Postgres>,
    author_id: Option<UserId>,
    draft_or_live: Option<DraftOrLive>,
    privacy_level: Vec<PrivacyLevel>,
    blocked: Option<bool>,
    page: i32,
    page_limit: u32,
    resource_types: Vec<Uuid>,
    order_by: Option<OrderBy>,
) -> sqlx::Result<Vec<ResourceResponse>> {
    let mut txn = db.begin().await?;

    let privacy_level: Vec<i16> = privacy_level.iter().map(|x| *x as i16).collect();

    let resource_data = sqlx::query!(
    //language=SQL
    r#"
with cte as (
    select array_agg(rd.id)
    from resource_data "rd"
          inner join resource on (draft_id = rd.id or (live_id = rd.id and rd.last_synced_at is not null and published_at is not null))
          left join resource_admin_data "admin" on admin.resource_id = resource.id
          left join resource_data_resource "rdr" on rd.id = rdr.resource_data_id
    where (author_id = $1 or $1 is null)
        and (blocked = $2 or $2 is null)
        and (rd.privacy_level = any($3) or $3 = array[]::smallint[])
        and (rdr.resource_type_id = any($4) or $4 = array[]::uuid[])
        and (draft_or_live = $5 or $5 is null)
    group by updated_at, created_at, resource.published_at, admin.resource_id
    order by case when $6 = 0 then created_at
        when $6 = 1 then published_at
        else coalesce(updated_at, created_at)
  end desc, resource_id
),
cte1 as (
    select * from unnest(array((select cte.array_agg[1] from cte))) with ordinality t(id
   , ord) order by ord
)
select resource.id                                              as "resource_id: ResourceId",
    privacy_level                                       as "privacy_level: PrivacyLevel",
    creator_id                                          as "creator_id?: UserId",
    author_id                                           as "author_id?: UserId",
    (select given_name || ' '::text || family_name
        from user_profile
     where user_profile.user_id = author_id)            as "author_name",
    created_at,
    updated_at,
    published_at,
    likes,
    views,
    live_up_to_date,
   display_name                                                                  as "display_name!",
   language                                                                      as "language!",
   description                                                                   as "description!",
   translated_description                                                        as "translated_description!: Json<HashMap<String,String>>",
   draft_or_live                                                                 as "draft_or_live!: DraftOrLive",
   (
       select row(resource_data_module.id, kind, is_complete)
       from resource_data_module
       where resource_data_id = resource_data.id
    )                                               as "cover?: (ModuleId, ModuleKind, bool)",
   array(select row (category_id)
         from resource_data_category
         where resource_data_id = resource_data.id)     as "categories!: Vec<(CategoryId,)>",
   array(select row (affiliation_id)
         from resource_data_affiliation
         where resource_data_id = resource_data.id)     as "affiliations!: Vec<(AffiliationId,)>",
   array(select row (age_range_id)
         from resource_data_age_range
         where resource_data_id = resource_data.id)     as "age_ranges!: Vec<(AgeRangeId,)>",
   array(
            select row (rdr.id, rdr.display_name, resource_type_id, resource_content)
            from resource_data_resource "rdr"
            where rdr.resource_data_id= resource_data.id
        )                                               as "additional_resource!: Vec<(AddId, String, TypeId, Value)>",
   locked                                     as "locked!",
   other_keywords                             as "other_keywords!",
   translated_keywords                        as "translated_keywords!",
   rating                                     as "rating!: Option<ResourceRating>",
   blocked                                    as "blocked!",
   curated                                    as "curated!"
from cte1
inner join resource_data on cte1.id = resource_data.id
inner join resource on (
    resource_data.id = resource.draft_id
    or (
        resource_data.id = resource.live_id
        and last_synced_at is not null
        and resource.published_at is not null
    )
)
left join resource_admin_data "admin" on admin.resource_id = resource.id
where ord > (1 * $7 * $8)
order by ord asc
limit $8
"#,
    author_id.map(|x| x.0),
    blocked,
    &privacy_level[..],
    &resource_types[..],
    draft_or_live.map(|it| it as i16),
    order_by.map(|it| it as i32),
    page,
    page_limit as i32,
)
    .fetch_all(&mut txn)
    .instrument(tracing::info_span!("query resource_data"))
    .await?;

    let v: Vec<_> = resource_data
        .into_iter()
        .map(|resource_data_row| ResourceResponse {
            id: resource_data_row.resource_id,
            published_at: resource_data_row.published_at,
            creator_id: resource_data_row.creator_id,
            author_id: resource_data_row.author_id,
            author_name: resource_data_row.author_name,
            likes: resource_data_row.likes,
            views: resource_data_row.views,
            live_up_to_date: resource_data_row.live_up_to_date,
            resource_data: ResourceData {
                created_at: resource_data_row.created_at,
                draft_or_live: resource_data_row.draft_or_live,
                display_name: resource_data_row.display_name,
                language: resource_data_row.language,
                cover: resource_data_row
                    .cover
                    .map(|(id, kind, is_complete)| LiteModule {
                        id,
                        kind,
                        is_complete,
                    }),
                categories: resource_data_row
                    .categories
                    .into_iter()
                    .map(|(it,)| it)
                    .collect(),
                last_edited: resource_data_row.updated_at,
                description: resource_data_row.description,

                age_ranges: resource_data_row
                    .age_ranges
                    .into_iter()
                    .map(|(it,)| it)
                    .collect(),
                affiliations: resource_data_row
                    .affiliations
                    .into_iter()
                    .map(|(it,)| it)
                    .collect(),
                additional_resources: resource_data_row
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
                privacy_level: resource_data_row.privacy_level,
                locked: resource_data_row.locked,
                other_keywords: resource_data_row.other_keywords,
                translated_keywords: resource_data_row.translated_keywords,
                translated_description: resource_data_row.translated_description.0,
            },
            admin_data: ResourceAdminData {
                rating: resource_data_row.rating,
                blocked: resource_data_row.blocked,
                curated: resource_data_row.curated,
            },
        })
        .collect();

    txn.rollback().await?;

    Ok(v)
}

pub async fn update_draft(
    pool: &PgPool,
    api_key: &Option<String>,
    id: ResourceId,
    display_name: Option<&str>,
    categories: Option<&[CategoryId]>,
    age_ranges: Option<&[AgeRangeId]>,
    affiliations: Option<&[AffiliationId]>,
    language: Option<&str>,
    description: Option<&str>,
    privacy_level: Option<PrivacyLevel>,
    other_keywords: Option<String>,
) -> Result<(), error::UpdateWithMetadata> {
    let mut txn = pool.begin().await?;

    let draft_id = sqlx::query!(
        //language=SQL
        r#"
select draft_id from resource join resource_data on resource.draft_id = resource_data.id where resource.id = $1 for update
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
update resource_data
set privacy_level = coalesce($2, privacy_level)
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
update resource_data
set description = $2,
    translated_description = '{}',
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
update resource_data
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

    if let Some(display_name) = display_name {
        sqlx::query!(
            r#"
update resource_data
set display_name = $2,
    translated_name = '{}',
    updated_at = now()
where id = $1 and $2 is distinct from display_name"#,
            draft_id,
            display_name,
        )
        .execute(&mut txn)
        .await?;
    }

    // update trivial, not null fields
    sqlx::query!(
        //language=SQL
        r#"
update resource_data
set language         = coalesce($2, language),
    updated_at = now()
where id = $1
  and ($2::text is not null and $2 is distinct from language)
"#,
        draft_id,
        language,
    )
    .execute(&mut txn)
    .await?;

    if let Some(categories) = categories {
        super::recycle_metadata(&mut txn, "resource_data", draft_id, categories)
            .await
            .map_err(super::meta::handle_metadata_err)?;
    }

    if let Some(affiliations) = affiliations {
        super::recycle_metadata(&mut txn, "resource_data", draft_id, affiliations)
            .await
            .map_err(super::meta::handle_metadata_err)?;
    }

    if let Some(age_ranges) = age_ranges {
        super::recycle_metadata(&mut txn, "resource_data", draft_id, age_ranges)
            .await
            .map_err(super::meta::handle_metadata_err)?;
    }

    txn.commit().await?;

    Ok(())
}

pub async fn delete(pool: &PgPool, id: ResourceId) -> Result<(), error::Delete> {
    let mut txn = pool.begin().await?;

    let (draft_id, live_id) = get_draft_and_live_ids(&mut txn, id)
        .await
        .ok_or(error::Delete::ResourceNotFound)?;

    sqlx::query!(
        //language=SQL
        r#"
with del_data as (
    delete from resource_data
        where id is not distinct from $1 or id is not distinct from $2)
delete
from resource
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
    blocked: Option<bool>,
    author_id: Option<UserId>,
    draft_or_live: Option<DraftOrLive>,
    resource_types: Vec<Uuid>,
) -> sqlx::Result<(u64, u64)> {
    let privacy_level: Vec<i16> = privacy_level.iter().map(|x| *x as i16).collect();

    let resource_data = sqlx::query!(
        //language=SQL
        r#"
        with cte as (
            select array_agg(rd.id)
            from resource_data "rd"
                  inner join resource on (draft_id = rd.id or (live_id = rd.id and rd.last_synced_at is not null and published_at is not null))
                  left join resource_admin_data "admin" on admin.resource_id = resource.id
                  left join resource_data_resource "rdr" on rd.id = rdr.resource_data_id
            where (rd.draft_or_live = $1 or $1 is null)
                and (author_id = $2 or $2 is null)
                and (blocked = $3 or $3 is null)
                and (rd.privacy_level = any($4) or $4 = array[]::smallint[])
                and (rdr.resource_type_id = any($5) or $5 = array[]::uuid[])
            group by updated_at, created_at, resource.published_at, admin.resource_id, resource_id
        )
            select count(*) as "count!" from unnest(array((select cte.array_agg[1] from cte))) with ordinality t(id
           , ord)
        "#,
        draft_or_live.map(|it| it as i16),
        author_id.map(|it| it.0),
        blocked,
        &privacy_level[..],
        &resource_types[..]
    )
    .fetch_one(db)
    .instrument(tracing::info_span!("count resource_data"))
    .await?;

    println!(
        "privacy: {:?}, blocked: {:?}, author_id: {:?}, version: {:?}, resource: {:?}",
        privacy_level, blocked, author_id, draft_or_live, resource_types
    );

    let resource = sqlx::query!(
        //language=SQL
        r#"
        with cte as (
            select array_agg(resource.id)
            from resource
                  inner join resource_data "rd" on (draft_id = rd.id or (live_id = rd.id and rd.last_synced_at is not null and published_at is not null))
                  left join resource_admin_data "admin" on admin.resource_id = resource.id
                  left join resource_data_resource "rdr" on rd.id = rdr.resource_data_id
            where (rd.draft_or_live = $1 or $1 is null)
                and (author_id = $2 or $2 is null)
                and (blocked = $3 or $3 is null)
                and (rd.privacy_level = any($4) or $4 = array[]::smallint[])
                and (rdr.resource_type_id = any($5) or $5 = array[]::uuid[])
            group by updated_at, created_at, resource.published_at, admin.resource_id, resource_id
        )
            select count(*) as "count!" from unnest(array((select cte.array_agg[1] from cte))) with ordinality t(id
           , ord)
        "#,
        draft_or_live.map(|it| it as i16),
        author_id.map(|it| it.0),
        blocked,
        &privacy_level[..],
        &resource_types[..]
    )
    .fetch_one(db)
    .instrument(tracing::info_span!("count resource"))
    .await?;

    Ok((resource.count as u64, resource_data.count as u64))
}

pub async fn count(db: &PgPool, privacy_level: PrivacyLevel) -> sqlx::Result<u64> {
    sqlx::query!(
        //language=SQL
        r#"
select count(*) as "count!: i64"
from resource_data
inner join resource on resource.live_id = resource_data.id
where (privacy_level = coalesce($1, privacy_level))
"#,
        privacy_level as i16,
    )
    .fetch_one(db)
    .await
    .map(|it| it.count as u64)
}

pub async fn get_draft_and_live_ids(
    txn: &mut PgConnection,
    resource_id: ResourceId,
) -> Option<(Uuid, Uuid)> {
    sqlx::query!(
        //language=SQL
        r#"
select draft_id, live_id from resource where id = $1
"#,
        resource_id.0
    )
    .fetch_optional(&mut *txn)
    .await
    .ok()?
    .map(|it| (it.draft_id, it.live_id))
}

/// Clones a copy of the resource data and modules, preserving the module's stable IDs
pub async fn clone_data(
    txn: &mut PgConnection,
    from_data_id: &Uuid,
    draft_or_live: DraftOrLive,
) -> Result<Uuid, error::CloneDraft> {
    let new_id = sqlx::query!(
        //language=SQL
        r#"
insert into resource_data
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
from resource_data
where id = $1
returning id
        "#,
        from_data_id,
    )
    .fetch_one(&mut *txn)
    .await?
    .id;

    update_draft_or_live(txn, new_id, draft_or_live).await?;

    // copy metadata
    sqlx::query!(
        //language=SQL
        r#"
insert into resource_data_module ("index", resource_data_id, kind, is_complete, contents)
select "index", $2 as "resource_id", kind, is_complete, contents
from resource_data_module
where resource_data_id = $1
        "#,
        from_data_id,
        new_id,
    )
    .execute(&mut *txn)
    .await?;

    sqlx::query!(
        //language=SQL
        r#"
insert into resource_data_resource(resource_data_id, resource_type_id, display_name, resource_content)
select $2, resource_type_id, display_name, resource_content
from resource_data_resource
where resource_data_id = $1
        "#,
        from_data_id,
        new_id,
    )
    .execute(&mut *txn)
    .await?;

    sqlx::query!(
        //language=SQL
        r#"
insert into resource_data_affiliation(resource_data_id, affiliation_id)
select $2, affiliation_id
from resource_data_affiliation
where resource_data_id = $1
        "#,
        from_data_id,
        new_id,
    )
    .execute(&mut *txn)
    .await?;

    sqlx::query!(
        //language=SQL
        r#"
insert into resource_data_age_range(resource_data_id, age_range_id)
select $2, age_range_id
from resource_data_age_range
where resource_data_id = $1
        "#,
        from_data_id,
        new_id,
    )
    .execute(&mut *txn)
    .await?;

    sqlx::query!(
        //language=SQL
        r#"
insert into resource_data_category(resource_data_id, category_id)
select $2, category_id
from resource_data_category
where resource_data_id = $1
        "#,
        from_data_id,
        new_id,
    )
    .execute(&mut *txn)
    .await?;

    // copy modules

    Ok(new_id)
}

pub async fn clone_resource(
    db: &PgPool,
    parent: ResourceId,
    user_id: UserId,
) -> Result<ResourceId, error::CloneDraft> {
    let mut txn = db.begin().await?;

    let (draft_id, live_id) = get_draft_and_live_ids(&mut *txn, parent)
        .await
        .ok_or(error::CloneDraft::ResourceNotFound)?;

    let new_draft_id = clone_data(&mut txn, &draft_id, DraftOrLive::Draft).await?;
    let new_live_id = clone_data(&mut txn, &live_id, DraftOrLive::Live).await?;

    let new_resource = sqlx::query!(
        //language=SQL
        r#"
insert into resource (creator_id, author_id, parents, live_id, draft_id)
select creator_id, $2, array_append(parents, $1), $3, $4
from resource
where id = $1
returning id as "id!: ResourceId"
"#,
        parent.0,
        user_id.0,
        new_live_id,
        new_draft_id,
    )
    .fetch_one(&mut txn)
    .await?;

    txn.commit().await?;

    Ok(new_resource.id)
}

pub async fn resource_view(db: &PgPool, id: ResourceId) -> anyhow::Result<()> {
    let mut txn = db.begin().await?;

    let resource = sqlx::query!(
        // language=SQL
        r#"
select published_at  as "published_at?"
from resource
where id = $1
    "#,
        id.0
    )
    .fetch_one(&mut txn)
    .await?;

    //check if resource has been published and playable
    if resource.published_at == None {
        return Err(anyhow::anyhow!("Resource has not been published"));
    };

    //update Resource view count
    sqlx::query!(
        // language=SQL
        r#"
update resource
set views = views + 1
where id = $1;
            "#,
        id.0,
    )
    .execute(db)
    .await?;

    txn.commit().await?;

    Ok(())
}

pub async fn update_admin_data(
    pool: &PgPool,
    resource_id: ResourceId,
    rating: Option<ResourceRating>,
    blocked: Option<bool>,
    curated: Option<bool>,
) -> Result<(), error::NotFound> {
    let mut txn = pool.begin().await?;

    if let Some(rating) = rating {
        sqlx::query!(
            //language=SQL
            r#"
update resource_admin_data
set rating = coalesce($2, rating)
where resource_id = $1 and $2 is distinct from rating
            "#,
            resource_id.0,
            rating as i16
        )
        .execute(&mut txn)
        .await?;
    }

    if let Some(blocked) = blocked {
        sqlx::query!(
            //language=SQL
            r#"
update resource_admin_data
set blocked = coalesce($2, blocked)
where resource_id = $1 and $2 is distinct from blocked
            "#,
            resource_id.0,
            blocked
        )
        .execute(&mut txn)
        .await?;

        sqlx::query!(
            //language=SQL
            r#"
update resource_data
set updated_at = now()
from resource
where resource.live_id = $1
            "#,
            resource_id.0,
        )
        .execute(&mut txn)
        .await?;
    }

    if let Some(curated) = curated {
        sqlx::query!(
            //language=SQL
            r#"
update resource_admin_data
set curated = coalesce($2, curated)
where resource_id = $1 and $2 is distinct from curated
            "#,
            resource_id.0,
            curated
        )
        .execute(&mut txn)
        .await?;
    }

    txn.commit().await?;

    Ok(())
}

pub async fn resource_like(db: &PgPool, user_id: UserId, id: ResourceId) -> anyhow::Result<()> {
    let mut txn = db.begin().await?;

    let resource = sqlx::query!(
        r#"
select author_id    "author_id: UserId",
       published_at  as "published_at?"
from resource
where id = $1
    "#,
        id.0
    )
    .fetch_one(&mut txn)
    .await?;

    //check if Resource is published and likeable
    if resource.published_at == None {
        return Err(anyhow::anyhow!("Resource has not been published"));
    };

    // check if current user is the author
    if resource.author_id == Some(user_id) {
        return Err(anyhow::anyhow!("Cannot like your own resource"));
    };

    // checks if user has already liked the resource
    sqlx::query!(
        // language=SQL
        r#"
insert into resource_like(resource_id, user_id)
values ($1, $2)
            "#,
        id.0,
        user_id.0
    )
    .execute(&mut txn)
    .await
    .map_err(|_| anyhow::anyhow!("Cannot like a resource more than once"))?;

    txn.commit().await?;

    Ok(())
}

pub async fn resource_unlike(db: &PgPool, user_id: UserId, id: ResourceId) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
delete from resource_like
where resource_id = $1 and user_id = $2
    "#,
        id.0,
        user_id.0
    )
    .execute(db)
    .await
    .map_err(|_| anyhow::anyhow!("Must like resource prior to unlike"))?;

    Ok(())
}

pub async fn resource_is_liked(db: &PgPool, user_id: UserId, id: ResourceId) -> sqlx::Result<bool> {
    let exists = sqlx::query!(
        r#"
select exists (
    select 1
    from resource_like
    where
        resource_id = $1
        and user_id = $2
) as "exists!"
    "#,
        id.0,
        user_id.0
    )
    .fetch_one(db)
    .await?
    .exists;

    Ok(exists)
}

pub async fn is_admin(db: &PgPool, user_id: UserId) -> Result<bool, error::Auth> {
    let authed = sqlx::query!(
        r#"
select exists(select 1 from user_scope where user_id = $1 and scope = any($2)) as "authed!"
"#,
        user_id.0,
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
    user_id: UserId,
    resource_id: Option<ResourceId>,
) -> Result<(), error::Auth> {
    let authed = match resource_id {
        None => {
            sqlx::query!(
                r#"
select exists(select 1 from user_scope where user_id = $1 and scope = any($2)) as "authed!"
"#,
                user_id.0,
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
    not exists (select 1 from resource where resource.id = $4 and resource.author_id <> $1)
) as "authed!"
"#,
                user_id.0,
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
    resource_data_id: Uuid,
    draft_or_live: DraftOrLive,
) -> sqlx::Result<()> {
    sqlx::query!(
        //language=SQL
        r#"
update resource_data
set draft_or_live = $2
where id = $1
            "#,
        resource_data_id,
        draft_or_live as i16
    )
    .execute(&mut *conn)
    .await?;

    Ok(())
}
