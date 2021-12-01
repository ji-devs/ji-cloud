use algolia::{
    filter::{AndFilter, AndFilterable, CommonFilter, FacetFilter, ScoredFacetFilter, TagFilter},
    request::{BatchWriteRequests, SearchQuery, VirtualKeyRestrictions},
    response::SearchResponse,
    ApiKey, AppId, Client as Inner,
};
use anyhow::Context;
use chrono::Utc;
use core::settings::AlgoliaSettings;
use futures::TryStreamExt;
use serde::Serialize;

use shared::{
    domain::{
        category::CategoryId,
        image::{ImageId, ImageKind},
        jig::{JigFocus, JigId, PrivacyLevel},
        meta::{AffiliationId, AgeRangeId, GoalId, ImageStyleId, ImageTagIndex, ResourceTypeId},
    },
    media::MediaGroupKind,
};
use sqlx::PgPool;
use std::convert::TryInto;
use uuid::Uuid;

use migration::ResyncKind;

mod migration;

const PREMIUM_TAG: &'static str = "premium";
const PUBLISHED_TAG: &'static str = "published"; // not currently used
const HAS_AUTHOR_TAG: &'static str = "hasAuthor";

#[derive(Serialize)]
struct BatchJig<'a> {
    name: &'a str,
    language: &'a str,
    age_ranges: &'a [Uuid],
    age_range_names: &'a [String],
    affiliations: &'a [Uuid],
    affiliation_names: &'a [String],
    additional_resources: &'a [Uuid],
    additional_resource_names: &'a [String],
    goals: &'a [Uuid],
    goal_names: &'a [String],
    categories: &'a [Uuid],
    category_names: &'a [String],
    author: Option<Uuid>,
    author_name: Option<String>,
    #[serde(rename = "_tags")]
    tags: Vec<&'static str>,
    jig_focus: &'a str,
}

#[derive(Serialize)]
struct BatchImage<'a> {
    name: &'a str,
    description: &'a str,
    translated_description: &'a str,
    styles: &'a [Uuid],
    style_names: &'a [String],
    age_ranges: &'a [Uuid],
    age_range_names: &'a [String],
    affiliations: &'a [Uuid],
    affiliation_names: &'a [String],
    categories: &'a [Uuid],
    category_names: &'a [String],
    image_tags: &'a [i16],
    image_tag_names: &'a [String],
    media_subkind: &'a str,
    #[serde(rename = "_tags")]
    tags: Vec<&'static str>,
}

#[derive(Serialize)]
#[serde(tag = "media_kind")]
#[serde(rename_all = "camelCase")]
enum BatchMedia<'a> {
    Image(BatchImage<'a>),
}

/// Manager for background task that reads updated jigs or media from the database, then
/// performs batch updates to the indices.
#[derive(Clone)]
pub struct Manager {
    pub db: PgPool,
    pub inner: Inner,
    pub media_index: String,
    pub jig_index: String,
}

impl Manager {
    pub fn new(settings: Option<AlgoliaSettings>, db: PgPool) -> anyhow::Result<Option<Self>> {
        let (app_id, key, media_index, jig_index) = match settings {
            Some(settings) => match (
                settings.management_key,
                settings.media_index,
                settings.jig_index,
            ) {
                (Some(key), Some(media_index), Some(jig_index)) => {
                    (settings.application_id, key, media_index, jig_index)
                }
                _ => return Ok(None),
            },
            None => return Ok(None),
        };

        Ok(Some(Self {
            inner: Inner::new(AppId::new(app_id), ApiKey(key))?,
            media_index,
            jig_index,
            db,
        }))
    }

    pub async fn spawn_cron_jobs(&self) -> anyhow::Result<()> {
        log::info!("reached updates for spawning jobs");

        for count in 0..2 {
            let res = if count == 0 {
                self.update_images()
                    .await
                    .context("update images task errored")
            } else {
                self.update_jigs().await.context("update jigs task errored")
            };

            match res {
                Ok(true) => {}
                Ok(false) => {
                    log::info!("exiting algolia indexing task (out of date)");
                }
                Err(e) => {
                    log::error!("{:?}", e);
                    sentry::integrations::anyhow::capture_anyhow(&e);
                }
            }
        }

        Ok(())
    }

    pub async fn migrate(&self) -> anyhow::Result<()> {
        let mut txn = self.db.begin().await?;

        let algolia_version = sqlx::query!(
            r#"
with new_row as (
    insert into "settings" default values on conflict(singleton) do nothing returning algolia_index_version    
)
select algolia_index_version as "algolia_index_version!" from new_row
union
select algolia_index_version as "algolia_index_version!" from "settings"
"#,
        )
        .fetch_one(&mut txn)
        .await?
        .algolia_index_version;

        if algolia_version == migration::INDEX_VERSION {
            return Ok(());
        }

        let migrations_to_run = &migration::INDEXING_MIGRATIONS[(algolia_version as usize)..];

        for (idx, (_, updater)) in migrations_to_run.iter().enumerate() {
            updater(&self.inner, &self.media_index, &self.jig_index)
                .await
                .with_context(|| {
                    anyhow::anyhow!(
                        "error while running algolia updater #{}",
                        idx + (algolia_version as usize) + 1
                    )
                })?;
        }

        // currently this can only be "no resync" or "complete resync" but eventually
        // we might want to be able to "resync everything that's only had an initial sync" or "everything that has had an update"
        let resync_mask =
            migrations_to_run
                .iter()
                .fold(ResyncKind::None, |acc, &(curr, _)| match (acc, curr) {
                    (_, ResyncKind::Complete) | (ResyncKind::Complete, _) => ResyncKind::Complete,
                    _ => ResyncKind::None,
                });

        match resync_mask {
            ResyncKind::Complete => {
                sqlx::query!("update image_metadata set last_synced_at = null")
                    .execute(&mut txn)
                    .await?;
            }
            ResyncKind::None => {}
        }

        sqlx::query!(
            r#"update "settings" set algolia_index_version = $1"#,
            migration::INDEX_VERSION
        )
        .execute(&mut txn)
        .await?;

        txn.commit().await?;

        Ok(())
    }

    async fn batch_media(&self, batch: BatchWriteRequests) -> anyhow::Result<Vec<Uuid>> {
        let resp = self.inner.batch(&self.media_index, &batch).await?;

        let ids: Result<Vec<_>, _> = resp
            .object_ids
            .into_iter()
            .map(|id| Uuid::parse_str(&id))
            .collect();

        Ok(ids?)
    }

    async fn batch_jigs(&self, batch: BatchWriteRequests) -> anyhow::Result<Vec<Uuid>> {
        let resp = self.inner.batch(&self.jig_index, &batch).await?;

        let ids: Result<Vec<_>, _> = resp
            .object_ids
            .into_iter()
            .map(|id| Uuid::parse_str(&id))
            .collect();

        Ok(ids?)
    }

    // FIXME drafts
    async fn update_jigs(&self) -> anyhow::Result<bool> {
        log::info!("reached update jigs");
        let mut txn = self.db.begin().await?;

        let is_outdated = sqlx::query!(
            r#"select algolia_index_version != $1 as "outdated!" from settings"#,
            migration::INDEX_VERSION
        )
        .fetch_one(&mut txn)
        .await?
        .outdated;

        if is_outdated {
            return Ok(false);
        }

        // todo: allow for some way to do a partial update (for example, by having a channel for queueing partial updates)
        let requests: Vec<_> = sqlx::query!(
            //language=SQL
            r#"
select jig.id,
       display_name                                                                                                 as "name",
       language                                                                                                     as "language!",
       array((select affiliation_id
              from jig_data_affiliation
              where jig_data_id = jig_data.id))                                                                     as "affiliations!",
       array((select affiliation.display_name
              from affiliation
                       inner join jig_data_affiliation on affiliation.id = jig_data_affiliation.affiliation_id
              where jig_data_affiliation.jig_data_id = jig_data.id))                                                as "affiliation_names!",
        array((select resource_type_id
                from jig_data_additional_resource
                where jig_data_id = jig_data.id))                                                                     as "additional_resources!",
        array((select resource_type.display_name
              from resource_type
                        inner join jig_data_additional_resource on resource_type.id = jig_data_additional_resource.resource_type_id
             where jig_data_additional_resource.jig_data_id = jig_data.id))                                         as "additional_resource_names!",
       array((select age_range_id
              from jig_data_age_range
              where jig_data_id = jig_data.id))                                                                     as "age_ranges!",
       array((select age_range.display_name
              from age_range
                       inner join jig_data_age_range on age_range.id = jig_data_age_range.age_range_id
              where jig_data_age_range.jig_data_id = jig_data.id))                                                  as "age_range_names!",
       array((select goal_id from jig_data_goal where jig_data_id = jig_data.id))                                   as "goals!",
       array((select goal.display_name
              from goal
                       inner join jig_data_goal on goal.id = jig_data_goal.goal_id
              where jig_data_goal.jig_data_id = jig_data.id))                                                       as "goal_names!",
       array((select category_id
              from jig_data_category
              where jig_data_id = jig_data.id))                                                                     as "categories!",
       array((select name
              from category
                       inner join jig_data_category on category.id = jig_data_category.category_id
              where jig_data_category.jig_data_id = jig_data.id))                                                   as "category_names!",
       privacy_level                                                                                                as "privacy_level!: PrivacyLevel",
       jig_focus                                                                                                    as "jig_focus!: JigFocus",
       author_id                                                                                                    as "author",
       (select given_name || ' '::text || family_name
        from user_profile
        where user_profile.user_id = jig.author_id)                                                                 as "author_name"

from jig
         inner join jig_data on live_id = jig_data.id
where last_synced_at is null
   or (updated_at is not null and last_synced_at < updated_at)
limit 100 for no key update skip locked;
     "#
        )
        .fetch(&mut txn)
        .map_ok(|row| {
            let mut tags = Vec::new();

            tags.push(row.privacy_level.as_str());

            if row.author.is_some() {
                tags.push(HAS_AUTHOR_TAG);
            }

            algolia::request::BatchWriteRequest::UpdateObject {
            body: match serde_json::to_value(&BatchJig {
                name: &row.name,
                language: &row.language,
                goals: &row.goals,
                goal_names: &row.goal_names,
                age_ranges: &row.age_ranges,
                age_range_names: &row.age_range_names,
                affiliations: &row.affiliations,
                affiliation_names: &row.affiliation_names,
                additional_resources: &row.additional_resources,
                additional_resource_names: &row.additional_resource_names,
                categories: &row.categories,
                category_names: &row.category_names,
                author: row.author,
                author_name: row.author_name,
                jig_focus: &row.jig_focus.as_str(),
                tags
            })
            .expect("failed to serialize BatchJig to json")
            {
                serde_json::Value::Object(map) => map,
                _ => panic!("failed to serialize BatchJig to json map"),
            },
            object_id: row.id.to_string(),
        }})
        .try_collect()
        .await?;

        if requests.is_empty() {
            log::warn!("Request is empty");
            return Ok(true);
        }

        log::debug!("Updating a batch of {} jigs(s)", requests.len());

        let request = algolia::request::BatchWriteRequests { requests };
        let ids = self.batch_jigs(request).await?;

        log::debug!("Updated a batch of {} jigs(s)", ids.len());

        sqlx::query!(
            //language=SQL
            r#"
update jig_data
set last_synced_at = now()
where jig_data.id = any (select live_id from jig where jig.id = any ($1))
"#,
            &ids
        )
        .execute(&mut txn)
        .await?;

        txn.commit().await?;

        log::info!("completed update jigs");

        Ok(true)
    }

    async fn update_images(&self) -> anyhow::Result<bool> {
        log::info!("reached update images");
        let mut txn = self.db.begin().await?;

        let is_outdated = sqlx::query!(
            r#"select algolia_index_version != $1 as "outdated!" from settings"#,
            migration::INDEX_VERSION
        )
        .fetch_one(&mut txn)
        .await?
        .outdated;

        if is_outdated {
            return Ok(false);
        }

        // todo: allow for some way to do a partial update (for example, by having a channel for queueing partial updates)
        let requests: Vec<_> = sqlx::query!(
            //language=SQL
            r#"
select id,
       name,
       kind                                                                                     as "kind!: ImageKind",
       description,
       translated_description                                                                   as "translated_description!",
       array((select affiliation_id from image_affiliation where image_id = image_metadata.id)) as "affiliations!",
       array((select affiliation.display_name
              from affiliation
                       inner join image_affiliation on affiliation.id = image_affiliation.affiliation_id
              where image_affiliation.image_id = image_metadata.id))                            as "affiliation_names!",
       array((select style_id from image_style where image_id = image_metadata.id))             as "styles!",
       array((select style.display_name
              from style
                       inner join image_style on style.id = image_style.style_id
              where image_style.image_id = image_metadata.id))                                  as "style_names!",
       array((select age_range_id from image_age_range where image_id = image_metadata.id))     as "age_ranges!",
       array((select age_range.display_name
              from age_range
                       inner join image_age_range on age_range.id = image_age_range.age_range_id
              where image_age_range.image_id = image_metadata.id))                              as "age_range_names!",
       array((select category_id from image_category where image_id = image_metadata.id))       as "categories!",
       array((select name
              from category
                       inner join image_category on category.id = image_category.category_id
              where image_category.image_id = image_metadata.id))                               as "category_names!",
       array((select index
              from image_tag
                       inner join image_tag_join on image_tag.index = image_tag_join.tag_index
              where image_tag_join.image_id = image_metadata.id))                               as "tags!",
       array((select display_name
              from image_tag
                       inner join image_tag_join on image_tag.index = image_tag_join.tag_index
              where image_tag_join.image_id = image_metadata.id))                               as "tag_names!",
       (publish_at < now() is true)                                                             as "is_published!",
       is_premium
from image_metadata
         join image_upload on id = image_id
where (last_synced_at is null or
       (updated_at is not null and last_synced_at < updated_at) or
       (publish_at < now() is true and last_synced_at < publish_at))
  and processed_at is not null
limit 100 for no key update skip locked;
     "#
        )
        .fetch(&mut txn)
        .map_ok(|row| {
            let mut tags = Vec::new();
            if row.is_published {
                tags.push(PUBLISHED_TAG);
            }

            if row.is_premium {
                tags.push(PREMIUM_TAG);
            }

            algolia::request::BatchWriteRequest::UpdateObject {
            body: match serde_json::to_value(&BatchMedia::Image(BatchImage {
                media_subkind: &row.kind.to_str(),
                name: &row.name,
                description: &row.description,
                translated_description: &row.translated_description,
                styles: &row.styles,
                style_names: &row.style_names,
                age_ranges: &row.age_ranges,
                age_range_names: &row.age_range_names,
                affiliations: &row.affiliations,
                affiliation_names: &row.affiliation_names,
                image_tags: &row.tags,
                image_tag_names: &row.tag_names,
                categories: &row.categories,
                category_names: &row.category_names,
                tags
            }))
            .expect("failed to serialize BatchImage to json")
            {
                serde_json::Value::Object(map) => map,
                _ => panic!("failed to serialize BatchImage to json map"),
            },
            object_id: row.id.to_string(),
        }})
        .try_collect()
        .await?;

        if requests.is_empty() {
            return Ok(true);
        }

        log::debug!("Updating a batch of {} image(s)", requests.len());

        let request = algolia::request::BatchWriteRequests { requests };
        let ids = self.batch_media(request).await?;

        log::debug!("Updated a batch of {} image(s)", ids.len());

        sqlx::query!(
            "update image_metadata set last_synced_at = now() where id = any($1)",
            &ids
        )
        .execute(&mut txn)
        .await?;

        txn.commit().await?;

        log::info!("completed update images");

        Ok(true)
    }
}

#[derive(Clone)]
pub struct SearchKeyStore {
    frontend_search_parent_key: ApiKey,
}

impl SearchKeyStore {
    pub fn new(frontend_search_parent_key: String) -> anyhow::Result<Self> {
        Ok(Self {
            frontend_search_parent_key: ApiKey(frontend_search_parent_key),
        })
    }

    pub fn generate_virtual_key(
        &self,
        user_id: Option<Uuid>,
        ttl: Option<chrono::Duration>,
    ) -> ApiKey {
        self.frontend_search_parent_key
            .generate_virtual_key(&VirtualKeyRestrictions {
                user_token: user_id.map(|u| u.to_string()),
                valid_until: ttl.map(|ttl| Utc::now() + ttl),
            })
    }
}

/// OR UUIDs then append them to AND filter for a named facet
fn filters_for_ids_or<T: Into<Uuid> + Copy>(
    filters: &mut Vec<Box<dyn AndFilterable>>,
    facet_name: &str,
    ids: &[T],
) {
    let mut or_filters = algolia::filter::OrFilter::<FacetFilter> { filters: vec![] };

    for id in ids.iter().copied() {
        let id: Uuid = id.into();

        // Push onto OR filter
        or_filters.filters.push(CommonFilter {
            filter: FacetFilter {
                facet_name: facet_name.to_owned(),
                value: id.to_string(),
            },
            invert: false,
        })
    }

    // (A or B) and (C or D)

    if !(or_filters.filters.is_empty()) {
        // append all OR filters to AND filter
        filters.push(Box::new(or_filters));
    }
}

/// OR ints then append them to AND filter for a named facet
fn filters_for_ints_or<T: Into<i64> + Copy>(
    filters: &mut Vec<Box<dyn AndFilterable>>,
    facet_name: &str,
    ints: &[T],
) {
    let mut or_filters = algolia::filter::OrFilter::<FacetFilter> { filters: vec![] };

    for v in ints {
        let v: i64 = (*v).into();
        // Push onto OR filter
        or_filters.filters.push(CommonFilter {
            filter: FacetFilter {
                facet_name: facet_name.to_owned(),
                value: v.to_string(),
            },
            invert: false,
        })
    }

    if !(or_filters.filters.is_empty()) {
        // append all OR filters to AND filter
        filters.push(Box::new(or_filters));
    }
}

/// Filter with ordered priority.
///
/// If using priority scoring, this can only rank the first 62 items.
/// This is because scores are weighted exponentially and i64::MAX = 2^63-1. 63 less one so that
/// it does not overflow when summed with lesser scores.
///
/// The remaining will be assigned a score of 1, which is the default score for all filters.
fn scored_int_filtering(facet_name: &str, ints: &[i64]) -> Vec<CommonFilter<ScoredFacetFilter>> {
    let mut filters = Vec::new();

    const I64_BITS: u32 = 64;
    let count = ints.len() as u32;

    // start with the score for the highest priority tag
    let mut score = match count > I64_BITS - 1 {
        true => 1_i64 << (I64_BITS - 2), // 2_i64.pow(i64::BITS - 1),
        false if count == 0 => return vec![],
        false => 1_i64 << (count - 1),
    };

    // computes the score for the next lower priority tag
    let next_score = |score: &mut i64| match *score > 1 {
        true => *score = *score >> 1,
        false => *score = 1,
    };

    for v in ints.iter() {
        filters.push(CommonFilter {
            filter: ScoredFacetFilter {
                facet_name: facet_name.to_owned(),
                value: v.to_string(),
                score,
            },
            invert: false,
        });

        next_score(&mut score)
    }

    filters
}

fn media_filter(kind: MediaGroupKind, invert: bool) -> CommonFilter<FacetFilter> {
    CommonFilter {
        filter: FacetFilter {
            facet_name: "media_kind".to_owned(),
            value: kind.to_str().to_owned(),
        },
        invert,
    }
}

/// Client for handling operations that operate on single objects during http requests, such as
/// searching or deleting a single item.
#[derive(Clone)]
pub struct Client {
    inner: Inner,
    media_index: String,
    jig_index: String,
}

impl Client {
    pub fn new(settings: Option<AlgoliaSettings>) -> anyhow::Result<Option<Self>> {
        if let Some(settings) = settings {
            let app_id = algolia::AppId::new(settings.application_id);

            let (inner, media_index, jig_index) = match (
                settings.backend_search_key,
                settings.media_index,
                settings.jig_index,
            ) {
                (Some(key), Some(media_index), Some(jig_index)) => {
                    (Inner::new(app_id, ApiKey(key))?, media_index, jig_index)
                }
                _ => return Ok(None),
            };

            Ok(Some(Self {
                inner,
                media_index,
                jig_index,
            }))
        } else {
            Ok(None)
        }
    }

    // todo: return ImageId (can't because of repr issues in sqlx)
    pub async fn search_image(
        &self,
        query: &str,
        page: Option<u32>,
        is_premium: Option<bool>,
        is_published: Option<bool>,
        styles: &[ImageStyleId],
        age_ranges: &[AgeRangeId],
        affiliations: &[AffiliationId],
        categories: &[CategoryId],
        tags: &[ImageTagIndex],
        tags_priority: &[ImageTagIndex],
    ) -> anyhow::Result<Option<(Vec<Uuid>, u32, u64)>> {
        let mut filters = algolia::filter::AndFilter {
            filters: vec![Box::new(media_filter(MediaGroupKind::Image, false))],
        };

        if let Some(is_published) = is_published {
            filters.filters.push(Box::new(CommonFilter {
                filter: TagFilter(PUBLISHED_TAG.to_owned()),
                invert: !is_published,
            }))
        }

        if let Some(is_premium) = is_premium {
            filters.filters.push(Box::new(CommonFilter {
                filter: TagFilter(PREMIUM_TAG.to_owned()),
                invert: !is_premium,
            }))
        }

        filters_for_ids_or(&mut filters.filters, "styles", styles);
        filters_for_ids_or(&mut filters.filters, "age_ranges", age_ranges);
        filters_for_ids_or(&mut filters.filters, "affiliations", affiliations);
        filters_for_ids_or(&mut filters.filters, "categories", categories);
        filters_for_ints_or(&mut filters.filters, "image_tags", tags);

        let optional_filters = scored_int_filtering(
            "image_tags",
            &tags_priority
                .iter()
                .map(|it| it.0 as i64)
                .collect::<Vec<i64>>(),
        );

        let results: SearchResponse = self
            .inner
            .search(
                &self.media_index,
                SearchQuery {
                    query: Some(query),
                    page,
                    get_ranking_info: true,
                    filters: Some(filters),
                    optional_filters: Some(optional_filters),
                    hits_per_page: None,
                    sum_or_filters_scores: true,
                },
            )
            .await?;

        let pages = results.page_count.try_into()?;
        let total_hits = results.hit_count as u64;

        let results = results
            .hits
            .into_iter()
            .map(|hit| hit.object_id.parse())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Some((results, pages, total_hits)))
    }

    pub async fn delete_image(&self, id: ImageId) {
        if let Err(e) = self.try_delete_image(id).await {
            log::warn!(
                "failed to delete image with id {} from algolia: {}",
                id.0.to_hyphenated(),
                e
            );
        }
    }

    pub async fn try_delete_image(&self, ImageId(id): ImageId) -> anyhow::Result<()> {
        self.inner
            .delete_object(&self.media_index, &id.to_string())
            .await?;

        Ok(())
    }

    pub async fn search_jig(
        &self,
        query: &str,
        page: Option<u32>,
        privacy_level: Option<PrivacyLevel>,
        language: Option<String>,
        age_ranges: &[AgeRangeId],
        affiliations: &[AffiliationId],
        additional_resources: &[ResourceTypeId],
        categories: &[CategoryId],
        goals: &[GoalId],
        author: Option<Uuid>,
        author_name: Option<String>,
        jig_focus: Option<JigFocus>,
    ) -> anyhow::Result<Option<(Vec<Uuid>, u32, u64)>> {
        let mut and_filters = algolia::filter::AndFilter { filters: vec![] };

        if let Some(author) = author {
            and_filters.filters.push(Box::new(CommonFilter {
                filter: TagFilter(HAS_AUTHOR_TAG.to_owned()),
                invert: false,
            }));

            and_filters.filters.push(Box::new(CommonFilter {
                filter: FacetFilter {
                    facet_name: "author".to_owned(),
                    value: author.to_string(),
                },
                invert: false,
            }))
        }

        if let Some(author_name) = author_name {
            and_filters.filters.push(Box::new(CommonFilter {
                filter: FacetFilter {
                    facet_name: "author_name".to_owned(),
                    value: author_name,
                },
                invert: false,
            }))
        }

        if let Some(jig_focus) = jig_focus {
            and_filters.filters.push(Box::new(CommonFilter {
                filter: TagFilter(jig_focus.as_str().to_owned()),
                invert: false,
            }))
        }

        if let Some(privacy_level) = privacy_level {
            and_filters.filters.push(Box::new(CommonFilter {
                filter: TagFilter(privacy_level.as_str().to_owned()),
                invert: false,
            }))
        }

        if let Some(language) = language {
            and_filters.filters.push(Box::new(CommonFilter {
                filter: FacetFilter {
                    facet_name: "language".to_owned(),
                    value: language,
                },
                invert: false,
            }))
        }

        filters_for_ids_or(&mut and_filters.filters, "age_ranges", age_ranges);
        filters_for_ids_or(&mut and_filters.filters, "affiliations", affiliations);
        filters_for_ids_or(
            &mut and_filters.filters,
            "additional_resources",
            additional_resources,
        );
        filters_for_ids_or(&mut and_filters.filters, "categories", categories);
        filters_for_ids_or(&mut and_filters.filters, "goals", goals);

        let results: SearchResponse = self
            .inner
            .search(
                &self.jig_index,
                SearchQuery::<'_, String, AndFilter> {
                    query: Some(query),
                    page,
                    get_ranking_info: true,
                    filters: Some(and_filters),
                    optional_filters: None,
                    hits_per_page: None,
                    sum_or_filters_scores: false,
                },
            )
            .await?;

        let pages = results.page_count.try_into()?;
        let total_hits = results.hit_count as u64;

        let results = results
            .hits
            .into_iter()
            .map(|hit| hit.object_id.parse())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Some((results, pages, total_hits)))
    }

    pub async fn delete_jig(&self, id: JigId) {
        if let Err(e) = self.try_delete_jig(id).await {
            log::warn!(
                "failed to delete jig with id {} from algolia: {}",
                id.0.to_hyphenated(),
                e
            );
        }
    }

    pub async fn try_delete_jig(&self, JigId(id): JigId) -> anyhow::Result<()> {
        self.inner
            .delete_object(&self.jig_index, &id.to_string())
            .await?;

        Ok(())
    }
}
