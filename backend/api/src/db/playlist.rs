use crate::translate::translate_text;
use anyhow::Context;
use serde_json::value::Value;
use shared::domain::playlist::{
    AdminPlaylistExport, PlaylistAdminData, PlaylistRating, PlaylistUpdateAdminDataRequest,
};
use shared::domain::{
    additional_resource::{AdditionalResource, AdditionalResourceId as AddId, ResourceContent},
    asset::{DraftOrLive, PrivacyLevel},
    category::CategoryId,
    jig::JigId,
    meta::{AffiliationId, AgeRangeId, ResourceTypeId as TypeId},
    module::{LiteModule, ModuleId, ModuleKind},
    playlist::{PlaylistData, PlaylistId, PlaylistResponse},
    user::{UserId, UserScope},
};
use sqlx::{types::Json, PgConnection, PgPool};
use std::collections::HashMap;
use tracing::{instrument, Instrument};
use uuid::Uuid;

use crate::error;

pub(crate) mod additional_resource;
pub(crate) mod module;

pub async fn create(
    pool: &PgPool,
    display_name: &str,
    categories: &[CategoryId],
    age_ranges: &[AgeRangeId],
    affiliations: &[AffiliationId],
    creator_id: UserId,
    language: &str,
    description: &str,
) -> Result<PlaylistId, CreatePlaylistError> {
    let mut txn = pool.begin().await?;

    let draft_id = create_playlist_data(
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

    let live_id = create_playlist_data(
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

    let playlist = sqlx::query!(
        //language=SQL
        r#"insert into playlist (creator_id, author_id, live_id, draft_id) values ($1, $1, $2, $3) returning id"#,
        creator_id.0,
        live_id,
        draft_id,
    )
    .fetch_one(&mut txn)
    .await?;

    txn.commit().await?;

    Ok(PlaylistId(playlist.id))
}

pub async fn create_playlist_data(
    txn: &mut PgConnection,
    display_name: &str,
    categories: &[CategoryId],
    age_ranges: &[AgeRangeId],
    affiliations: &[AffiliationId],
    language: &str,
    description: &str,
    draft_or_live: DraftOrLive,
) -> Result<Uuid, CreatePlaylistError> {
    log::warn!("description: {}", description);

    let playlist_data = sqlx::query!(
        // language=SQL
        r#"
insert into playlist_data
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

    super::recycle_metadata(&mut *txn, "playlist_data", playlist_data.id, categories).await?;

    super::recycle_metadata(&mut *txn, "playlist_data", playlist_data.id, age_ranges).await?;

    super::recycle_metadata(&mut *txn, "playlist_data", playlist_data.id, affiliations).await?;

    Ok(playlist_data.id)
}

pub enum CreatePlaylistError {
    Sqlx(sqlx::Error),
    InternalServerError(anyhow::Error),
}

impl From<sqlx::Error> for CreatePlaylistError {
    fn from(e: sqlx::Error) -> Self {
        Self::Sqlx(e)
    }
}

impl From<anyhow::Error> for CreatePlaylistError {
    fn from(e: anyhow::Error) -> Self {
        Self::InternalServerError(e)
    }
}

pub async fn get_one(
    pool: &PgPool,
    id: PlaylistId,
    draft_or_live: DraftOrLive,
    user_id: Option<UserId>,
) -> anyhow::Result<Option<PlaylistResponse>> {
    let res = sqlx::query!( //language=SQL
        r#"
with cte as (
    select id      as "playlist_id",
           creator_id,
           author_id,
           likes,
           plays,
           live_up_to_date,
           case
               when $2 = 0 then playlist.draft_id
               when $2 = 1 then playlist.live_id
               end as "draft_or_live_id",
           published_at,
           rating,
           blocked,
           curated,
           is_premium
    from playlist
    left join playlist_admin_data "admin" on admin.playlist_id = playlist.id
    where id = $1
)
select cte.playlist_id                                          as "playlist_id: PlaylistId",
       display_name,
       creator_id                                             as "creator_id?: UserId",
       author_id                                              as "author_id?: UserId",
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
       live_up_to_date,
       other_keywords,
       translated_keywords,
       rating                                               as "rating?: PlaylistRating",
       blocked                                              as "blocked",
       curated,
       is_premium                                           as "premium",
       exists(select 1 from playlist_like where playlist_id = $1 and user_id = $3) as "is_liked!",
       (
            select row(playlist_data_module.id, kind, is_complete)
            from playlist_data_module
            where playlist_data_id = cte.draft_or_live_id and "index" = 0
            order by "index"
        )                                                   as "cover?: (ModuleId, ModuleKind, bool)",
       array(select row (category_id)
             from playlist_data_category
             where playlist_data_id = cte.draft_or_live_id)     as "categories!: Vec<(CategoryId,)>",
       array(select row (affiliation_id)
             from playlist_data_affiliation
             where playlist_data_id = cte.draft_or_live_id)     as "affiliations!: Vec<(AffiliationId,)>",
       array(select row (age_range_id)
             from playlist_data_age_range
             where playlist_data_id = cte.draft_or_live_id)     as "age_ranges!: Vec<(AgeRangeId,)>",
       array(
             select row (jdar.id, jdar.display_name, resource_type_id, resource_content)
             from playlist_data_resource "jdar"
             where jdar.playlist_data_id = cte.draft_or_live_id
       )                                                    as "additional_resource!: Vec<(AddId, String, TypeId, Value)>",
       array(
           select row(jig_id)
           from playlist_data_jig
           where playlist_data_id = cte.draft_or_live_id
           order by "index"
       )                                                     as "items!: Vec<(JigId,)>"
from playlist_data
         inner join cte on cte.draft_or_live_id = playlist_data.id
"#,
        id.0,
        draft_or_live as i16,
        user_id.map(|x| x.0)
    )
        .fetch_optional(pool).await?;

    let playlist = res.map(|row| PlaylistResponse {
        id: row.playlist_id,
        published_at: row.published_at,
        creator_id: row.creator_id,
        author_id: row.author_id,
        author_name: row.author_name,
        likes: row.likes,
        plays: row.plays,
        live_up_to_date: row.live_up_to_date,
        is_liked: row.is_liked,
        playlist_data: PlaylistData {
            draft_or_live,
            display_name: row.display_name,
            language: row.language,
            categories: row.categories.into_iter().map(|(it,)| it).collect(),
            last_edited: row.updated_at,
            description: row.description,
            cover: row.cover.map(|(id, kind, is_complete)| LiteModule {
                id,
                kind,
                is_complete,
            }),
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
            items: row.items.into_iter().map(|(it,)| it).collect(),
        },
        admin_data: PlaylistAdminData {
            rating: row.rating,
            blocked: row.blocked,
            curated: row.curated,
            premium: row.premium,
        },
    });

    Ok(playlist)
}

pub async fn get_by_ids(
    db: &PgPool,
    ids: &[Uuid],
    draft_or_live: DraftOrLive,
    user_id: Option<UserId>,
) -> sqlx::Result<Vec<PlaylistResponse>> {
    let mut txn = db.begin().await?;

    let playlist = sqlx::query!(
        //language=SQL
        r#"
select playlist.id                                       as "id!: PlaylistId",
       creator_id                               as "creator_id?: UserId",
       author_id                                as "author_id?: UserId",
       (select given_name || ' '::text || family_name
        from user_profile
        where user_profile.user_id = author_id) as "author_name",
       live_id                                  as "live_id!",
       draft_id                                 as "draft_id!",
       published_at,
       likes                                    as "likes!",
       plays                                    as "plays!",
       live_up_to_date                          as "live_up_to_date!",
       exists(select 1 from playlist_like where playlist_id = playlist.id and user_id = $2) as "is_liked!",
       rating                                   as "rating?: PlaylistRating",
       blocked                                  as "blocked!",
       curated                                  as "curated!",
       is_premium                               as "premium!"
from playlist
inner join unnest($1::uuid[])
    with ordinality t(id, ord) using (id)
inner join playlist_admin_data "admin" on admin.playlist_id = playlist.id
order by ord asc
    "#,
        ids,
        user_id.map(|x| x.0)
    )
    .fetch_all(&mut txn)
    .await?;

    let playlist_data_ids: Vec<Uuid> = match draft_or_live {
        DraftOrLive::Draft => playlist.iter().map(|it| it.draft_id).collect(),
        DraftOrLive::Live => playlist.iter().map(|it| it.live_id).collect(),
    };

    let playlist_data = sqlx::query!(
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
        (
            select row(playlist_data_module.id, kind, is_complete)
            from playlist_data_module
            where playlist_data_id = playlist_data.id and "index" = 0
            order by "index"
        )                                                   as "cover?: (ModuleId, ModuleKind, bool)",
        array(select row (category_id)
            from playlist_data_category
            where playlist_data_id = playlist_data.id)     as "categories!: Vec<(CategoryId,)>",
        array(select row (affiliation_id)
            from playlist_data_affiliation
            where playlist_data_id = playlist_data.id)     as "affiliations!: Vec<(AffiliationId,)>",
        array(select row (age_range_id)
            from playlist_data_age_range
            where playlist_data_id = playlist_data.id)     as "age_ranges!: Vec<(AgeRangeId,)>",
        array(
            select row (jdar.id, jdar.display_name, resource_type_id, resource_content)
            from playlist_data_resource "jdar"
            where jdar.playlist_data_id = playlist_data.id
        )                                                    as "additional_resource!: Vec<(AddId, String, TypeId, Value)>",
        array(
            select row(jig_id)
            from playlist_data_jig
            where playlist_data_jig.playlist_data_id = playlist_data.id
            order by "index"
        )                                                     as "items!: Vec<(JigId,)>"
from playlist_data
inner join unnest($1::uuid[])
    with ordinality t(id, ord) using (id)
order by ord asc
"#,
        &playlist_data_ids
    )
        .fetch_all(&mut txn)
        .await?;

    let v = playlist
        .into_iter()
        .zip(playlist_data.into_iter())
        .map(|(playlist_row, playlist_data_row)| PlaylistResponse {
            id: playlist_row.id,
            published_at: playlist_row.published_at,
            creator_id: playlist_row.creator_id,
            author_id: playlist_row.author_id,
            author_name: playlist_row.author_name,
            likes: playlist_row.likes,
            plays: playlist_row.plays,
            live_up_to_date: playlist_row.live_up_to_date,
            is_liked: playlist_row.is_liked,
            playlist_data: PlaylistData {
                draft_or_live,
                display_name: playlist_data_row.display_name,
                language: playlist_data_row.language,
                cover: playlist_data_row
                    .cover
                    .map(|(id, kind, is_complete)| LiteModule {
                        id,
                        kind,
                        is_complete,
                    }),
                categories: playlist_data_row
                    .categories
                    .into_iter()
                    .map(|(it,)| it)
                    .collect(),
                last_edited: playlist_data_row.updated_at,
                description: playlist_data_row.description,
                age_ranges: playlist_data_row
                    .age_ranges
                    .into_iter()
                    .map(|(it,)| it)
                    .collect(),
                affiliations: playlist_data_row
                    .affiliations
                    .into_iter()
                    .map(|(it,)| it)
                    .collect(),
                additional_resources: playlist_data_row
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
                privacy_level: playlist_data_row.privacy_level,
                other_keywords: playlist_data_row.other_keywords,
                translated_keywords: playlist_data_row.translated_keywords,
                translated_description: playlist_data_row.translated_description.0,
                items: playlist_data_row
                    .items
                    .into_iter()
                    .map(|(it,)| it)
                    .collect(),
            },
            admin_data: PlaylistAdminData {
                rating: playlist_row.rating,
                blocked: playlist_row.blocked,
                curated: playlist_row.curated,
                premium: playlist_row.premium,
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
    page: i32,
    page_limit: u32,
    resource_types: Vec<Uuid>,
    user_id: Option<UserId>,
) -> sqlx::Result<Vec<PlaylistResponse>> {
    let mut txn = db.begin().await?;

    let privacy_level: Vec<i16> = privacy_level.iter().map(|x| *x as i16).collect();

    let playlist_data = sqlx::query!(
    //language=SQL
    r#"
with cte as (
    select (array_agg(cd.id))[1]
    from playlist_data "cd"
          left join playlist on (draft_id = cd.id or (live_id = cd.id and cd.last_synced_at is not null and published_at is not null))
          left join playlist_data_resource "resource" on cd.id = resource.playlist_data_id
    where (author_id = $1 or $1 is null)
        and (cd.draft_or_live = $2 or $2 is null)
        and (cd.privacy_level = any($3) or $3 = array[]::smallint[])
        and (resource.resource_type_id = any($4) or $4 = array[]::uuid[])
    group by coalesce(updated_at, created_at)
    order by coalesce(updated_at, created_at) desc
),
cte1 as (
    select * from unnest(array(select cte.array_agg from cte)) with ordinality t(id
   , ord) order by ord
)
select playlist.id                                                                as "playlist_id: PlaylistId",
    privacy_level                                                               as "privacy_level: PrivacyLevel",
    creator_id                                                                  as "creator_id?: UserId",
    author_id                                                                   as "author_id?: UserId",
    (select given_name || ' '::text || family_name
     from user_profile
     where user_profile.user_id = author_id)                                     as "author_name",
    published_at,
    likes,
    plays,
    live_up_to_date,
    exists(select 1 from playlist_like where playlist_id = playlist.id and user_id = $7)    as "is_liked!",
    display_name                                                                  as "display_name!",
    updated_at,
    language                                                                      as "language!",
    description                                                                   as "description!",
    translated_description                                                        as "translated_description!: Json<HashMap<String,String>>",
    draft_or_live                                                                 as "draft_or_live!: DraftOrLive",
    other_keywords                                                                as "other_keywords!",
    translated_keywords                                                           as "translated_keywords!",
    rating                                     as "rating?: PlaylistRating",
    blocked                                    as "blocked!",
    curated                                    as "curated!",
    is_premium                                 as "premium!",
    (
        select row(playlist_data_module.id, kind, is_complete)
        from playlist_data_module
        where playlist_data_id = playlist_data.id and "index" = 0
        order by "index"
    )                                                   as "cover?: (ModuleId, ModuleKind, bool)",
    array(select row (category_id)
            from playlist_data_category
            where playlist_data_id = playlist_data.id)     as "categories!: Vec<(CategoryId,)>",
    array(select row (affiliation_id)
            from playlist_data_affiliation
            where playlist_data_id = playlist_data.id)          as "affiliations!: Vec<(AffiliationId,)>",
    array(select row (age_range_id)
            from playlist_data_age_range
            where playlist_data_id = playlist_data.id)          as "age_ranges!: Vec<(AgeRangeId,)>",
    array(select row (id, display_name, resource_type_id, resource_content)
                from playlist_data_resource
                where playlist_data_id = playlist_data.id
          )                                          as "additional_resource!: Vec<(AddId, String, TypeId, Value)>",
    array(
        select row(jig_id)
        from playlist_data_jig
        where playlist_data_jig.playlist_data_id = playlist_data.id
        order by "index"
    )                                                     as "items!: Vec<(JigId,)>"
from cte1
inner join playlist_data on cte1.id = playlist_data.id
inner join playlist on (
    playlist_data.id = playlist.draft_id
    or (
        playlist_data.id = playlist.live_id
        and last_synced_at is not null
        and playlist.published_at is not null
    )
)
left join playlist_admin_data "admin" on admin.playlist_id = playlist.id
where ord > (1 * $5 * $6)
order by ord asc
limit $6
"#,
    author_id.map(|it| it.0),
    draft_or_live.map(|it| it as i16),
    &privacy_level[..],
    &resource_types[..],
    page,
    page_limit as i32,
    user_id.map(|x| x.0)
)
    .fetch_all(&mut txn)
    .instrument(tracing::info_span!("query playlist_data"))
    .await?;

    let v: Vec<_> = playlist_data
        .into_iter()
        .map(|playlist_data_row| PlaylistResponse {
            id: playlist_data_row.playlist_id,
            published_at: playlist_data_row.published_at,
            creator_id: playlist_data_row.creator_id,
            author_id: playlist_data_row.author_id,
            author_name: playlist_data_row.author_name,
            likes: playlist_data_row.likes,
            plays: playlist_data_row.plays,
            live_up_to_date: playlist_data_row.live_up_to_date,
            is_liked: playlist_data_row.is_liked,
            playlist_data: PlaylistData {
                draft_or_live: playlist_data_row.draft_or_live,
                display_name: playlist_data_row.display_name,
                language: playlist_data_row.language,
                cover: playlist_data_row
                    .cover
                    .map(|(id, kind, is_complete)| LiteModule {
                        id,
                        kind,
                        is_complete,
                    }),
                categories: playlist_data_row
                    .categories
                    .into_iter()
                    .map(|(it,)| it)
                    .collect(),
                last_edited: playlist_data_row.updated_at,
                description: playlist_data_row.description,
                age_ranges: playlist_data_row
                    .age_ranges
                    .into_iter()
                    .map(|(it,)| it)
                    .collect(),
                affiliations: playlist_data_row
                    .affiliations
                    .into_iter()
                    .map(|(it,)| it)
                    .collect(),
                additional_resources: playlist_data_row
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
                privacy_level: playlist_data_row.privacy_level,
                other_keywords: playlist_data_row.other_keywords,
                translated_keywords: playlist_data_row.translated_keywords,
                translated_description: playlist_data_row.translated_description.0,
                items: playlist_data_row
                    .items
                    .into_iter()
                    .map(|(it,)| it)
                    .collect(),
            },
            admin_data: PlaylistAdminData {
                rating: playlist_data_row.rating,
                blocked: playlist_data_row.blocked,
                curated: playlist_data_row.curated,
                premium: playlist_data_row.premium,
            },
        })
        .collect();

    txn.rollback().await?;

    Ok(v)
}

pub async fn update_draft(
    pool: &PgPool,
    api_key: &Option<String>,
    id: PlaylistId,
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
select draft_id from playlist join playlist_data on playlist.draft_id = playlist_data.id where playlist.id = $1 for update
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
update playlist_data
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
update playlist_data
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
update playlist_data
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
update playlist_data
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
update playlist_data
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
        super::recycle_metadata(&mut txn, "playlist_data", draft_id, categories)
            .await
            .map_err(super::meta::handle_metadata_err)?;
    }

    if let Some(affiliations) = affiliations {
        super::recycle_metadata(&mut txn, "playlist_data", draft_id, affiliations)
            .await
            .map_err(super::meta::handle_metadata_err)?;
    }

    if let Some(age_ranges) = age_ranges {
        super::recycle_metadata(&mut txn, "playlist_data", draft_id, age_ranges)
            .await
            .map_err(super::meta::handle_metadata_err)?;
    }

    if let Some(jig) = jig_ids {
        super::recycle_metadata(&mut txn, "playlist_data", draft_id, jig)
            .await
            .map_err(super::meta::handle_metadata_err)?;
    }

    txn.commit().await?;

    Ok(())
}

pub async fn delete(pool: &PgPool, id: PlaylistId) -> Result<(), error::Delete> {
    let mut txn = pool.begin().await?;

    let (draft_id, live_id) = get_draft_and_live_ids(&mut txn, id)
        .await
        .ok_or(error::Delete::ResourceNotFound)?;

    sqlx::query!(
        //language=SQL
        r#"
    update user_asset_data 
    set playlist_count = playlist_count - 1,
        total_asset_count = total_asset_count - 1
    from playlist
    where author_id = user_id and
          published_at is not null and 
          id = $1
          "#,
        id.0
    )
    .execute(&mut *txn)
    .await?;

    sqlx::query!(
        //language=SQL
        r#"
with del_data as (
    delete from playlist_data
        where id is not distinct from $1 or id is not distinct from $2)
delete
from playlist
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
    author_id: Option<UserId>,
    draft_or_live: Option<DraftOrLive>,
    resource_types: Vec<Uuid>,
) -> sqlx::Result<(u64, u64)> {
    let privacy_level: Vec<i16> = privacy_level.iter().map(|x| *x as i16).collect();

    let playlist_data = sqlx::query!(
        //language=SQL
        r#"
        with cte as (
            select (array_agg(cd.id))[1]
            from playlist_data "cd"
                  inner join playlist on (draft_id = cd.id or (live_id = cd.id and cd.last_synced_at is not null and published_at is not null))
                  left join playlist_data_resource "resource" on cd.id = resource.playlist_data_id
            where (author_id = $1 or $1 is null)
                and (cd.draft_or_live = $2 or $2 is null)
                and (cd.privacy_level = any($3) or $3 = array[]::smallint[])
                and (resource.resource_type_id = any($4) or $4 = array[]::uuid[])
            group by coalesce(updated_at, created_at)
        )
        select count(*) as "count!" from unnest(array(select cte.array_agg from cte)) with ordinality t(id, ord)
    "#,
        author_id.map(|it| it.0),
        draft_or_live.map(|it| it as i16),
        &privacy_level[..],
        &resource_types[..]
    )
    .fetch_one(db)
    .await?;

    let playlist = sqlx::query!(
        //language=SQL
        r#"
        with cte as (
            select (array_agg(playlist.id))[1]
            from playlist_data "cd"
                  inner join playlist on (draft_id = cd.id or (live_id = cd.id and cd.last_synced_at is not null and published_at is not null))
                  left join playlist_data_resource "resource" on cd.id = resource.playlist_data_id
            where (author_id = $1 or $1 is null)
                and (cd.draft_or_live = $2 or $2 is null)
                and (cd.privacy_level = any($3) or $3 = array[]::smallint[])
                and (resource.resource_type_id = any($4) or $4 = array[]::uuid[])
            group by coalesce(updated_at, created_at)
        )
        select count(*) as "count!" from unnest(array(select cte.array_agg from cte)) with ordinality t(id, ord)
"#,
        author_id.map(|it| it.0),
        draft_or_live.map(|it| it as i16),
        &privacy_level[..],
        &resource_types[..]
    )
    .fetch_one(db)
    .await?;

    Ok((playlist.count as u64, playlist_data.count as u64))
}

pub async fn get_draft_and_live_ids(
    txn: &mut PgConnection,
    playlist_id: PlaylistId,
) -> Option<(Uuid, Uuid)> {
    sqlx::query!(
        //language=SQL
        r#"
select draft_id, live_id from playlist where id = $1
"#,
        playlist_id.0,
    )
    .fetch_optional(&mut *txn)
    .await
    .ok()?
    .map(|it| (it.draft_id, it.live_id))
}

/// Clones a copy of the playlist data and modules, preserving the module's stable IDs
pub async fn clone_data(
    txn: &mut PgConnection,
    from_data_id: &Uuid,
    draft_or_live: DraftOrLive,
) -> Result<Uuid, error::CloneDraft> {
    println!("here in clone");
    let new_id = sqlx::query!(
        //language=SQL
        r#"
insert into playlist_data
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
from playlist_data
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
 insert into playlist_data_module ("index", playlist_data_id, kind, is_complete, contents)
 select "index", $2 as "playlist_id", kind, is_complete, contents
 from playlist_data_module
 where playlist_data_id = $1
            "#,
        from_data_id,
        new_id,
    )
    .execute(&mut *txn)
    .await?;

    sqlx::query!(
        //language=SQL
        r#"
insert into playlist_data_resource(playlist_data_id, resource_type_id, display_name, resource_content)
select $2, resource_type_id, display_name, resource_content
from playlist_data_resource
where playlist_data_id = $1
        "#,
        from_data_id,
        new_id,
    )
    .execute(&mut *txn)
    .await?;

    sqlx::query!(
        //language=SQL
        r#"
insert into playlist_data_affiliation(playlist_data_id, affiliation_id)
select $2, affiliation_id
from playlist_data_affiliation
where playlist_data_id = $1
        "#,
        from_data_id,
        new_id,
    )
    .execute(&mut *txn)
    .await?;

    sqlx::query!(
        //language=SQL
        r#"
insert into playlist_data_age_range(playlist_data_id, age_range_id)
select $2, age_range_id
from playlist_data_age_range
where playlist_data_id = $1
        "#,
        from_data_id,
        new_id,
    )
    .execute(&mut *txn)
    .await?;

    sqlx::query!(
        //language=SQL
        r#"
insert into playlist_data_category(playlist_data_id, category_id)
select $2, category_id
from playlist_data_category
where playlist_data_id = $1
        "#,
        from_data_id,
        new_id,
    )
    .execute(&mut *txn)
    .await?;

    sqlx::query!(
        //language=SQL
        r#"
insert into playlist_data_jig(playlist_data_id, jig_id, index)
select $2, jig_id, index
from playlist_data_jig
where playlist_data_id = $1
        "#,
        from_data_id,
        new_id,
    )
    .execute(&mut *txn)
    .await?;

    Ok(new_id)
}

pub async fn authz(
    db: &PgPool,
    user_id: UserId,
    playlist_id: Option<PlaylistId>,
) -> Result<(), error::Auth> {
    let authed = match playlist_id {
        None => {
            sqlx::query!(
                r#"
select exists(select 1 from user_scope where user_id = $1 and scope = any($2)) as "authed!"
"#,
                user_id.0,
                &[
                    UserScope::Admin as i16,
                    UserScope::AdminAsset as i16,
                    UserScope::ManageSelfAsset as i16,
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
    not exists (select 1 from playlist where playlist.id = $4 and playlist.author_id <> $1)
) as "authed!"
"#,
                user_id.0,
                &[UserScope::Admin as i16, UserScope::AdminAsset as i16,][..],
                UserScope::ManageSelfAsset as i16,
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
    playlist_data_id: Uuid,
    draft_or_live: DraftOrLive,
) -> sqlx::Result<()> {
    sqlx::query!(
        //language=SQL
        r#"
update playlist_data
set draft_or_live = $2
where id = $1
            "#,
        playlist_data_id,
        draft_or_live as i16
    )
    .execute(&mut *conn)
    .await?;

    Ok(())
}

pub async fn clone_playlist(
    db: &PgPool,
    parent: PlaylistId,
    user_id: UserId,
) -> Result<PlaylistId, error::CloneDraft> {
    let mut txn = db.begin().await?;

    let (draft_id, live_id) = get_draft_and_live_ids(&mut *txn, parent)
        .await
        .ok_or(error::CloneDraft::ResourceNotFound)?;

    let new_draft_id = clone_data(&mut txn, &draft_id, DraftOrLive::Draft).await?;
    let new_live_id = clone_data(&mut txn, &live_id, DraftOrLive::Live).await?;

    let new_playlist = sqlx::query!(
        //language=SQL
        r#"
insert into playlist (creator_id, author_id, parents, live_id, draft_id)
select creator_id, $2, array_append(parents, $1), $3, $4
from playlist
where id = $1
returning id as "id!: PlaylistId"
"#,
        parent.0,
        user_id.0,
        new_live_id,
        new_draft_id,
    )
    .fetch_one(&mut txn)
    .await?;

    txn.commit().await?;

    Ok(new_playlist.id)
}

pub async fn playlist_unlike(db: &PgPool, user_id: UserId, id: PlaylistId) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
delete from playlist_like
where playlist_id = $1 and user_id = $2
    "#,
        id.0,
        user_id.0
    )
    .execute(db)
    .await
    .map_err(|_| anyhow::anyhow!("Must like playlist prior to unlike"))?;

    Ok(())
}

pub async fn playlist_is_liked(db: &PgPool, user_id: UserId, id: PlaylistId) -> sqlx::Result<bool> {
    let exists = sqlx::query!(
        r#"
select exists (
    select 1
    from playlist_like
    where
        playlist_id = $1
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

pub async fn playlist_like(db: &PgPool, user_id: UserId, id: PlaylistId) -> anyhow::Result<()> {
    let mut txn = db.begin().await?;

    let playlist = sqlx::query!(
        r#"
select author_id    as "author_id: UserId",
       published_at  as "published_at?"
from playlist
where id = $1
    "#,
        id.0
    )
    .fetch_one(&mut txn)
    .await?;

    //check if Playlist is published and likeable
    if playlist.published_at == None {
        return Err(anyhow::anyhow!("Playlist has not been published"));
    };

    // check if current user is the author
    if playlist.author_id == Some(user_id) {
        return Err(anyhow::anyhow!("Cannot like your own playlist"));
    };

    // checks if user has already liked the playlist
    sqlx::query!(
        // language=SQL
        r#"
insert into playlist_like(playlist_id, user_id)
values ($1, $2)
            "#,
        id.0,
        user_id.0
    )
    .execute(&mut txn)
    .await
    .map_err(|_| anyhow::anyhow!("Cannot like a playlist more than once"))?;

    txn.commit().await?;

    Ok(())
}

pub async fn playlist_play(db: &PgPool, id: PlaylistId) -> anyhow::Result<()> {
    let mut txn = db.begin().await?;

    let playlist = sqlx::query!(
        // language=SQL
        r#"
select published_at  as "published_at?"
from playlist
where id = $1
    "#,
        id.0
    )
    .fetch_one(&mut txn)
    .await?;

    //check if playlist has been published and playable
    if playlist.published_at == None {
        return Err(anyhow::anyhow!("Playlist has not been published"));
    };

    //update Playlist play count
    sqlx::query!(
        // language=SQL
        r#"
update playlist
set plays = plays + 1
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
    playlist_id: PlaylistId,
    admin_data: PlaylistUpdateAdminDataRequest,
) -> Result<(), error::NotFound> {
    let mut txn = pool.begin().await?;

    let blocked = admin_data.blocked.into_option();

    sqlx::query!(
        // language=SQL
        r#"
update playlist_admin_data
set
    rating = coalesce($2, rating),
    blocked = coalesce($3, blocked),
    curated = coalesce($4, curated),
    is_premium = coalesce($5, is_premium)
where playlist_id = $1
"#,
        playlist_id.0,
        admin_data.rating.into_option() as Option<PlaylistRating>,
        blocked,
        admin_data.curated.into_option(),
        admin_data.premium.into_option(),
    )
    .execute(&mut txn)
    .await?;

    if blocked.is_some() {
        sqlx::query!(
            //language=SQL
            r#"
update playlist_data
set updated_at = now()
from playlist
where playlist.live_id = $1
            "#,
            playlist_id.0,
        )
        .execute(&mut txn)
        .await?;
    }

    txn.commit().await?;

    Ok(())
}

pub async fn playlists_export(db: &sqlx::PgPool) -> anyhow::Result<Vec<AdminPlaylistExport>> {
    let rows = sqlx::query!(
        //language=SQL
        r#"
        with cte as (
            select array_agg(pd.id)
            from playlist_data "pd"
                  inner join playlist on (draft_id = pd.id or (live_id = pd.id and pd.last_synced_at is not null and published_at is not null))
                  left join playlist_admin_data "admin" on admin.playlist_id = playlist.id
            where (pd.draft_or_live = $1)
            group by updated_at, created_at, playlist.published_at, admin.playlist_id
        ),
        cte1 as (
            select * from unnest(array((select cte.array_agg[1] from cte))) with ordinality t(id
           , ord) order by ord
        )
        select playlist.id                                      as "playlist_id: PlaylistId",
            privacy_level                                       as "privacy_level: PrivacyLevel",
            creator_id                                          as "creator_id?: UserId",
            author_id                                           as "author_id?: UserId",
            (select given_name || ' '::text || family_name
                from user_profile
             where user_profile.user_id = author_id)            as "author_name",
            created_at,
            published_at,
            likes,
            plays,
            display_name                                        as "display_name!",
            language                                            as "language!",
            description                                         as "description!",
            rating                                              as "rating!: Option<PlaylistRating>",
            blocked                                             as "blocked!",
            curated                                             as "curated!",
            is_premium                                          as "premium!"
        from cte1
        inner join playlist_data on cte1.id = playlist_data.id
        inner join playlist on (
            playlist_data.id = playlist.draft_id
            or (
                playlist_data.id = playlist.live_id
                and last_synced_at is not null
                and playlist.published_at is not null
            )
        )
        left join playlist_admin_data "admin" on admin.playlist_id = playlist.id
        "#,
        DraftOrLive::Live as i16
    )
        .fetch_all(db)
        .instrument(tracing::info_span!("query jig_data for export"))
        .await?;

    Ok(rows
        .into_iter()
        .map(|row| AdminPlaylistExport {
            id: row.playlist_id,
            description: row.description,
            display_name: row.display_name,
            premium: row.premium,
            blocked: row.blocked,
            author_id: row.author_id,
            author_name: row.author_name,
            likes: row.likes,
            plays: row.plays,
            rating: row.rating,
            privacy_level: row.privacy_level,
            created_at: row.created_at,
            published_at: row.published_at,
            language: row.language,
        })
        .collect())
}
