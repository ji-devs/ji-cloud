use crate::translate::{multi_translation, translate_text};
use anyhow::Context;
use serde_json::{json, value::Value};
use shared::domain::{
    category::CategoryId,
    jig::{
        additional_resource::{AdditionalResource, AdditionalResourceId as AddId, ResourceContent},
        module::{body::ThemeId, ModuleId},
        AudioBackground, AudioEffects, AudioFeedbackNegative, AudioFeedbackPositive,
        DeleteUserJigs, DraftOrLive, JigAdminData, JigData, JigFocus, JigId, JigPlayerSettings,
        JigRating, JigResponse, LiteModule, ModuleKind, PrivacyLevel, TextDirection,
    },
    meta::{AffiliationId, AgeRangeId, GoalId, ResourceTypeId as TypeId},
    user::UserScope,
};
use sqlx::{types::Json, PgConnection, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

use crate::error;

pub(crate) mod additional_resource;
pub(crate) mod curation;
pub(crate) mod module;
pub(crate) mod player;
pub(crate) mod report;

pub async fn create(
    pool: &PgPool,
    api_key: &Option<String>,
    display_name: &str,
    goals: &[GoalId],
    categories: &[CategoryId],
    age_ranges: &[AgeRangeId],
    affiliations: &[AffiliationId],
    creator_id: Uuid,
    language: &str,
    description: &str,
    default_player_settings: &JigPlayerSettings,
    jig_focus: &JigFocus,
) -> Result<JigId, CreateJigError> {
    let mut txn = pool.begin().await?;

    let translated_description: Option<HashMap<String, String>> = if !description.is_empty() {
        let translate_text = match &api_key {
            Some(key) => multi_translation(description, key)
                .await
                .context("could not translate text")?,
            None => None,
        };
        translate_text
    } else {
        None
    };

    let draft_id = create_jig_data(
        &mut txn,
        display_name,
        goals,
        categories,
        age_ranges,
        affiliations,
        language,
        description,
        default_player_settings,
        DraftOrLive::Draft,
        &translated_description,
    )
    .await?;

    let live_id = create_jig_data(
        &mut txn,
        display_name,
        goals,
        categories,
        age_ranges,
        affiliations,
        language,
        description,
        default_player_settings,
        DraftOrLive::Live,
        &translated_description,
    )
    .await?;

    let jig = sqlx::query!(
        //language=SQL
        r#"insert into jig (creator_id, author_id, live_id, draft_id, jig_focus) values ($1, $1, $2, $3, $4) returning id"#,
        creator_id,
        live_id,
        draft_id,
        (*jig_focus) as i16,
    )
    .fetch_one(&mut txn)
    .await?;

    sqlx::query!(
        // language=SQL
        r#"
insert into jig_play_count (jig_id, play_count)
values ($1, 0)
        "#,
        jig.id
    )
    .execute(&mut txn)
    .await?;

    txn.commit().await?;

    Ok(JigId(jig.id))
}

/// If `default_module_stable_ids` is `Some` is given, then insert the default modules with those stable IDs.
///
/// Otherwise if `None`, then create new stable ids and return them as a part of the response.
pub async fn create_jig_data(
    txn: &mut PgConnection, // FIXME does this work?
    display_name: &str,
    goals: &[GoalId],
    categories: &[CategoryId],
    age_ranges: &[AgeRangeId],
    affiliations: &[AffiliationId],
    language: &str,
    description: &str,
    default_player_settings: &JigPlayerSettings,
    draft_or_live: DraftOrLive,
    translated_description: &Option<HashMap<String, String>>,
) -> Result<Uuid, CreateJigError> {
    log::warn!("description: {}", description);

    let jig_data = sqlx::query!(
        // language=SQL
        r#"
insert into jig_data
   (display_name, language, description, direction, display_score, track_assessments, drag_assist, draft_or_live)
values ($1, $2, $3, $4, $5, $6, $7, $8)
returning id
"#,
        display_name,
        language,
        description,
        default_player_settings.direction as i16,
        default_player_settings.display_score,
        default_player_settings.track_assessments,
        default_player_settings.drag_assist,
        draft_or_live as i16,
    )
    .fetch_one(&mut *txn)
    .await?;

    if let Some(translation) = translated_description {
        sqlx::query!(
            // language=SQL
            r#"
    update jig_data
    set translated_description = $2::jsonb
    where id = $1
    "#,
            &jig_data.id,
            json!(translation)
        )
        .execute(&mut *txn)
        .await?;
    }

    super::recycle_metadata(&mut *txn, "jig_data", jig_data.id, goals).await?;
    super::recycle_metadata(&mut *txn, "jig_data", jig_data.id, categories).await?;
    super::recycle_metadata(&mut *txn, "jig_data", jig_data.id, age_ranges).await?;
    super::recycle_metadata(&mut *txn, "jig_data", jig_data.id, affiliations).await?;

    Ok(jig_data.id)
}

/// Handle errors for creating a module when posting a Jig
/// This is here because the scope is limited to the above function
pub enum CreateJigError {
    Sqlx(sqlx::Error),
    DefaultModules(serde_json::Error),
    InternalServerError(anyhow::Error),
}

impl From<sqlx::Error> for CreateJigError {
    fn from(e: sqlx::Error) -> Self {
        Self::Sqlx(e)
    }
}

impl From<anyhow::Error> for CreateJigError {
    fn from(e: anyhow::Error) -> Self {
        Self::InternalServerError(e)
    }
}

impl From<serde_json::Error> for CreateJigError {
    fn from(e: serde_json::Error) -> Self {
        Self::DefaultModules(e)
    }
}

pub async fn get_one(
    pool: &PgPool,
    id: JigId,
    draft_or_live: DraftOrLive,
) -> anyhow::Result<Option<JigResponse>> {
    let res = sqlx::query!( //language=SQL
        r#"
with cte as (
    select id      as "jig_id",
           creator_id,
           author_id,
           liked_count,
           play_count,
           case
               when $2 = 0 then jig.draft_id
               when $2 = 1 then jig.live_id
               end as "draft_or_live_id",
           published_at,
           rating,
           blocked,
           curated,
           jig_focus
    from jig
    left join jig_play_count on jig_play_count.jig_id = jig.id
    left join jig_admin_data "admin" on admin.jig_id = jig.id
    where id = $1
)
select cte.jig_id                                          as "jig_id: JigId",
       display_name,
       creator_id,
       author_id,
       (select given_name || ' '::text || family_name
        from user_profile
        where user_profile.user_id = author_id)            as "author_name",
       published_at,
       updated_at,
       privacy_level                                       as "privacy_level!: PrivacyLevel",
       jig_focus                                           as "jig_focus!: JigFocus",
       language,
       description,
       translated_description                              as "translated_description!: Json<HashMap<String, String>>",
       direction                                           as "direction: TextDirection",
       display_score,
       track_assessments,
       drag_assist,
       theme                                               as "theme: ThemeId",
       audio_background                                    as "audio_background: AudioBackground",
       liked_count,
       play_count,
       locked,
       other_keywords,
       translated_keywords,
       rating                                               as "rating?: JigRating",
       blocked                                              as "blocked",
       curated,
       array(select row (unnest(audio_feedback_positive))) as "audio_feedback_positive!: Vec<(AudioFeedbackPositive,)>",
       array(select row (unnest(audio_feedback_negative))) as "audio_feedback_negative!: Vec<(AudioFeedbackNegative,)>",
       array(
               select row (jig_data_module.id, kind, is_complete)
               from jig_data_module
               where jig_data_id = cte.draft_or_live_id
               order by "index"
           )                                               as "modules!: Vec<(ModuleId, ModuleKind, bool)>",
       array(select row (goal_id)
             from jig_data_goal
             where jig_data_id = cte.draft_or_live_id)     as "goals!: Vec<(GoalId,)>",
       array(select row (category_id)
             from jig_data_category
             where jig_data_id = cte.draft_or_live_id)     as "categories!: Vec<(CategoryId,)>",
       array(select row (affiliation_id)
             from jig_data_affiliation
             where jig_data_id = cte.draft_or_live_id)     as "affiliations!: Vec<(AffiliationId,)>",
       array(select row (age_range_id)
             from jig_data_age_range
             where jig_data_id = cte.draft_or_live_id)     as "age_ranges!: Vec<(AgeRangeId,)>",
       array(
             select row (jdar.id, jdar.display_name, resource_type_id, resource_content)
             from jig_data_additional_resource "jdar"
             where jdar.jig_data_id = cte.draft_or_live_id
       )                                                    as "additional_resource!: Vec<(AddId, String, TypeId, Value)>"
from jig_data
         inner join cte on cte.draft_or_live_id = jig_data.id
"#,
        id.0,
        draft_or_live as i16,
    )
        .fetch_optional(pool).await?;

    let jig = res.map(|row| JigResponse {
        id: row.jig_id,
        published_at: row.published_at,
        creator_id: row.creator_id,
        author_id: row.author_id,
        author_name: row.author_name,
        likes: row.liked_count,
        plays: row.play_count,
        jig_focus: row.jig_focus,
        jig_data: JigData {
            draft_or_live,
            display_name: row.display_name,
            language: row.language,
            modules: row
                .modules
                .into_iter()
                .map(|(id, kind, is_complete)| LiteModule {
                    id,
                    kind,
                    is_complete,
                })
                .collect(),
            goals: row.goals.into_iter().map(|(it,)| it).collect(),
            categories: row.categories.into_iter().map(|(it,)| it).collect(),
            last_edited: row.updated_at,
            description: row.description,
            default_player_settings: JigPlayerSettings {
                direction: row.direction,
                display_score: row.display_score,
                track_assessments: row.track_assessments,
                drag_assist: row.drag_assist,
            },
            theme: row.theme,
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
            audio_background: row.audio_background,
            audio_effects: AudioEffects {
                feedback_positive: row
                    .audio_feedback_positive
                    .into_iter()
                    .map(|(it,)| it)
                    .collect(),
                feedback_negative: row
                    .audio_feedback_negative
                    .into_iter()
                    .map(|(it,)| it)
                    .collect(),
            },
            privacy_level: row.privacy_level,
            locked: row.locked,
            other_keywords: row.other_keywords,
            translated_keywords: row.translated_keywords,
            translated_description: row.translated_description.0,
        },
        admin_data: JigAdminData {
            rating: row.rating,
            blocked: row.blocked,
            curated: row.curated,
        },
    });

    Ok(jig)
}

pub async fn get_by_ids(
    db: &PgPool,
    ids: &[Uuid],
    draft_or_live: DraftOrLive,
) -> sqlx::Result<Vec<JigResponse>> {
    let mut txn = db.begin().await?;

    let jig = sqlx::query!(
        //language=SQL
        r#"
select jig.id                                       as "id!: JigId",
       creator_id,
       author_id                                as "author_id",
       (select given_name || ' '::text || family_name
        from user_profile
        where user_profile.user_id = author_id) as "author_name",
       live_id                                  as "live_id!",
       draft_id                                 as "draft_id!",
       published_at,
       liked_count                              as "liked_count!",
       (
           select play_count
           from jig_play_count
           where jig_play_count.jig_id = jig.id
       )                                        as "play_count!",
       rating                                   as "rating?: JigRating",
       blocked                                  as "blocked!",
       curated                                  as "curated!",
       jig_focus                                as "jig_focus!: JigFocus"
from jig
         inner join unnest($1::uuid[])
    with ordinality t(id, ord) using (id)
    inner join jig_admin_data "admin" on admin.jig_id = jig.id
where blocked = false
    "#,
        ids,
    )
    .fetch_all(&mut txn)
    .await?;

    let jig_data_ids: Vec<Uuid> = match draft_or_live {
        DraftOrLive::Draft => jig.iter().map(|it| it.draft_id).collect(),
        DraftOrLive::Live => jig.iter().map(|it| it.live_id).collect(),
    };

    let jig_data = sqlx::query!(
        //language=SQL
        r#"
select id,
       display_name                                                                  as "display_name!",
       updated_at,
       language                                                                      as "language!",
       description                                                                   as "description!",
       translated_description                                                        as "translated_description!: Json<HashMap<String,String>>",
       direction                                                                     as "direction!: TextDirection",
       display_score                                                                 as "display_score!",
       track_assessments                                                             as "track_assessments!",
       drag_assist                                                                   as "drag_assist!",
       theme                                                                         as "theme!: ThemeId",
       audio_background                                                              as "audio_background!: Option<AudioBackground>",
       array(select row (unnest(audio_feedback_positive)))                           as "audio_feedback_positive!: Vec<(AudioFeedbackPositive,)>",
       array(select row (unnest(audio_feedback_negative)))                           as "audio_feedback_negative!: Vec<(AudioFeedbackNegative,)>",
       array(
               select row (jig_data_module.id, kind, is_complete)
               from jig_data_module
               where jig_data_id = jig_data.id
               order by "index"
           )                                               as "modules!: Vec<(ModuleId, ModuleKind, bool)>",
       array(select row (goal_id)
             from jig_data_goal
             where jig_data_id = jig_data.id)     as "goals!: Vec<(GoalId,)>",
       array(select row (category_id)
             from jig_data_category
             where jig_data_id = jig_data.id)     as "categories!: Vec<(CategoryId,)>",
       array(select row (affiliation_id)
             from jig_data_affiliation
             where jig_data_id = jig_data.id)     as "affiliations!: Vec<(AffiliationId,)>",
       array(select row (age_range_id)
             from jig_data_age_range
             where jig_data_id = jig_data.id)     as "age_ranges!: Vec<(AgeRangeId,)>",
       array(
                select row (jdar.id, jdar.display_name, resource_type_id, resource_content)
                from jig_data_additional_resource "jdar"
                where jdar.jig_data_id = jig_data.id
            )                                               as "additional_resource!: Vec<(AddId, String, TypeId, Value)>",
       privacy_level                              as "privacy_level!: PrivacyLevel",
       locked                                     as "locked!",
       other_keywords                             as "other_keywords!",
       translated_keywords                        as "translated_keywords!"
from jig_data
         inner join unnest($1::uuid[])
    with ordinality t(id, ord) using (id)
where draft_or_live is not null
"#,
        &jig_data_ids
    )
        .fetch_all(&mut txn).await?;

    let v = jig
        .into_iter()
        .zip(jig_data.into_iter())
        .map(|(jig_row, jig_data_row)| JigResponse {
            id: jig_row.id,
            published_at: jig_row.published_at,
            creator_id: jig_row.creator_id,
            author_id: jig_row.author_id,
            author_name: jig_row.author_name,
            likes: jig_row.liked_count,
            plays: jig_row.play_count,
            jig_focus: jig_row.jig_focus,
            jig_data: JigData {
                draft_or_live,
                display_name: jig_data_row.display_name,
                language: jig_data_row.language,
                modules: jig_data_row
                    .modules
                    .into_iter()
                    .map(|(id, kind, is_complete)| LiteModule {
                        id,
                        kind,
                        is_complete,
                    })
                    .collect(),
                goals: jig_data_row.goals.into_iter().map(|(it,)| it).collect(),
                categories: jig_data_row
                    .categories
                    .into_iter()
                    .map(|(it,)| it)
                    .collect(),
                last_edited: jig_data_row.updated_at,
                description: jig_data_row.description,
                default_player_settings: JigPlayerSettings {
                    direction: jig_data_row.direction,
                    display_score: jig_data_row.display_score,
                    track_assessments: jig_data_row.track_assessments,
                    drag_assist: jig_data_row.drag_assist,
                },
                theme: jig_data_row.theme,
                age_ranges: jig_data_row
                    .age_ranges
                    .into_iter()
                    .map(|(it,)| it)
                    .collect(),
                affiliations: jig_data_row
                    .affiliations
                    .into_iter()
                    .map(|(it,)| it)
                    .collect(),
                additional_resources: jig_data_row
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
                audio_background: jig_data_row.audio_background,
                audio_effects: AudioEffects {
                    feedback_positive: jig_data_row
                        .audio_feedback_positive
                        .into_iter()
                        .map(|(it,)| it)
                        .collect(),
                    feedback_negative: jig_data_row
                        .audio_feedback_negative
                        .into_iter()
                        .map(|(it,)| it)
                        .collect(),
                },
                privacy_level: jig_data_row.privacy_level,
                locked: jig_data_row.locked,
                other_keywords: jig_data_row.other_keywords,
                translated_keywords: jig_data_row.translated_keywords,
                translated_description: jig_data_row.translated_description.0,
            },
            admin_data: JigAdminData {
                rating: jig_row.rating,
                blocked: jig_row.blocked,
                curated: jig_row.curated,
            },
        })
        .collect();

    txn.rollback().await?;

    Ok(v)
}

pub async fn browse(
    db: &sqlx::Pool<sqlx::Postgres>,
    author_id: Option<Uuid>,
    jig_focus: Option<JigFocus>,
    draft_or_live: Option<DraftOrLive>,
    page: i32,
) -> sqlx::Result<(Vec<JigResponse>, u64)> {
    let mut txn = db.begin().await?;

    let jig = sqlx::query!(
        //language=SQL
        r#"
    select live_id                                  as "live_id!",
           draft_id                                 as "draft_id!"
    from jig
        left join jig_admin_data "admin" on admin.jig_id = jig.id
        left join jig_data "jig_dt" on jig_dt.id = jig.draft_id
        left join jig_data "jig_tt" on jig_tt.id = jig.live_id
    where blocked = false
        and (jig_dt.draft_or_live is not null and jig_tt.draft_or_live is not null)
        and ((jig_dt.draft_or_live = $3 or $3 is null)
              or (jig_tt.draft_or_live = $3 or $3 is null))
        and (author_id = $1 or $1 is null)
        and (jig_focus = $2 or $2 is null)
        "#,
        author_id,
        jig_focus.map(|it| it as i16),
        draft_or_live.map(|it| it as i16)
    )
    .fetch_all(&mut txn)
    .await?;

    let jig_data_ids: Vec<Uuid> = if let Some(draft_or_live) = draft_or_live {
        let choose = match draft_or_live {
            DraftOrLive::Draft => jig.iter().map(|it| it.draft_id).collect(),
            DraftOrLive::Live => jig.iter().map(|it| it.live_id).collect(),
        };
        choose
    } else {
        let draft_ids: Vec<Uuid> = jig.iter().map(|it| it.draft_id).collect();
        let live_ids: Vec<Uuid> = jig.iter().map(|it| it.live_id).collect();

        [&draft_ids[..], &live_ids[..]].concat()
    };

    let count = jig_data_ids.len() as u64;

    //TODO: purge junk jig data from with draft_or_live is NULL
    let jig_data = sqlx::query!(
        //language=SQL
        r#"
with cte as (
    select * from unnest($1::uuid[]) with ordinality t(id, ord) order by ord asc
)
select  jig.id                                              as "jig_id: JigId",
        privacy_level                                       as "privacy_level: PrivacyLevel",
        jig_focus                                           as "jig_focus!: JigFocus",
        creator_id,
        author_id,
        (select given_name || ' '::text || family_name
         from user_profile
         where user_profile.user_id = author_id)            as "author_name",
        published_at,
        liked_count,
        (
             select play_count
             from jig_play_count
             where jig_play_count.jig_id = jig.id
        )                                                   as "play_count!",
       display_name                                                                  as "display_name!",
       updated_at,
       language                                                                      as "language!",
       description                                                                   as "description!",
       translated_description                                                        as "translated_description!: Json<HashMap<String,String>>",
       direction                                                                     as "direction!: TextDirection",
       display_score                                                                 as "display_score!",
       track_assessments                                                             as "track_assessments!",
       drag_assist                                                                   as "drag_assist!",
       theme                                                                         as "theme!: ThemeId",
       audio_background                                                              as "audio_background!: Option<AudioBackground>",
       draft_or_live                                                                 as "draft_or_live!: DraftOrLive",
       array(select row (unnest(audio_feedback_positive)))                           as "audio_feedback_positive!: Vec<(AudioFeedbackPositive,)>",
       array(select row (unnest(audio_feedback_negative)))                           as "audio_feedback_negative!: Vec<(AudioFeedbackNegative,)>",
       array(
               select row (jig_data_module.id, kind, is_complete)
               from jig_data_module
               where jig_data_id = jig_data.id
               order by "index"
           )                                               as "modules!: Vec<(ModuleId, ModuleKind, bool)>",
       array(select row (goal_id)
             from jig_data_goal
             where jig_data_id = jig_data.id)     as "goals!: Vec<(GoalId,)>",
       array(select row (category_id)
             from jig_data_category
             where jig_data_id = jig_data.id)     as "categories!: Vec<(CategoryId,)>",
       array(select row (affiliation_id)
             from jig_data_affiliation
             where jig_data_id = jig_data.id)     as "affiliations!: Vec<(AffiliationId,)>",
       array(select row (age_range_id)
             from jig_data_age_range
             where jig_data_id = jig_data.id)     as "age_ranges!: Vec<(AgeRangeId,)>",
       array(
                select row (jdar.id, jdar.display_name, resource_type_id, resource_content)
                from jig_data_additional_resource "jdar"
                where jdar.jig_data_id = jig_data.id
            )                                               as "additional_resource!: Vec<(AddId, String, TypeId, Value)>",
       locked                                     as "locked!",
       other_keywords                             as "other_keywords!",
       translated_keywords                        as "translated_keywords!",
       rating                                     as "rating!: Option<JigRating>",
       blocked                                    as "blocked!",
       curated                                    as "curated!"
from jig_data
    inner join cte on cte.id = jig_data.id
    inner join jig on jig_data.id = jig.draft_id or jig_data.id = jig.live_id
    inner join jig_admin_data "admin" on admin.jig_id = jig.id
where cte.ord > (20 * $2)
order by ord asc
limit 20
"#,
        &jig_data_ids,
        page
    )
        .fetch_all(&mut txn).await?;

    let v = jig_data
        .into_iter()
        .map(|jig_data_row| JigResponse {
            id: jig_data_row.jig_id,
            published_at: jig_data_row.published_at,
            creator_id: jig_data_row.creator_id,
            author_id: jig_data_row.author_id,
            author_name: jig_data_row.author_name,
            likes: jig_data_row.liked_count,
            plays: jig_data_row.play_count,
            jig_focus: jig_data_row.jig_focus,
            jig_data: JigData {
                draft_or_live: jig_data_row.draft_or_live,
                display_name: jig_data_row.display_name,
                language: jig_data_row.language,
                modules: jig_data_row
                    .modules
                    .into_iter()
                    .map(|(id, kind, is_complete)| LiteModule {
                        id,
                        kind,
                        is_complete,
                    })
                    .collect(),
                goals: jig_data_row.goals.into_iter().map(|(it,)| it).collect(),
                categories: jig_data_row
                    .categories
                    .into_iter()
                    .map(|(it,)| it)
                    .collect(),
                last_edited: jig_data_row.updated_at,
                description: jig_data_row.description,
                default_player_settings: JigPlayerSettings {
                    direction: jig_data_row.direction,
                    display_score: jig_data_row.display_score,
                    track_assessments: jig_data_row.track_assessments,
                    drag_assist: jig_data_row.drag_assist,
                },
                theme: jig_data_row.theme,
                age_ranges: jig_data_row
                    .age_ranges
                    .into_iter()
                    .map(|(it,)| it)
                    .collect(),
                affiliations: jig_data_row
                    .affiliations
                    .into_iter()
                    .map(|(it,)| it)
                    .collect(),
                additional_resources: jig_data_row
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
                audio_background: jig_data_row.audio_background,
                audio_effects: AudioEffects {
                    feedback_positive: jig_data_row
                        .audio_feedback_positive
                        .into_iter()
                        .map(|(it,)| it)
                        .collect(),
                    feedback_negative: jig_data_row
                        .audio_feedback_negative
                        .into_iter()
                        .map(|(it,)| it)
                        .collect(),
                },
                privacy_level: jig_data_row.privacy_level,
                locked: jig_data_row.locked,
                other_keywords: jig_data_row.other_keywords,
                translated_keywords: jig_data_row.translated_keywords,
                translated_description: jig_data_row.translated_description.0,
            },
            admin_data: JigAdminData {
                rating: jig_data_row.rating,
                blocked: jig_data_row.blocked,
                curated: jig_data_row.curated,
            },
        })
        .collect();

    txn.rollback().await?;

    Ok((v, count))
}

pub async fn update_draft(
    pool: &PgPool,
    api_key: &Option<String>,
    id: JigId,
    display_name: Option<&str>,
    goals: Option<&[GoalId]>,
    categories: Option<&[CategoryId]>,
    age_ranges: Option<&[AgeRangeId]>,
    affiliations: Option<&[AffiliationId]>,
    language: Option<&str>,
    description: Option<&str>,
    default_player_settings: Option<&JigPlayerSettings>,
    theme: Option<&ThemeId>,
    audio_background: Option<&Option<AudioBackground>>,
    audio_effects: Option<&AudioEffects>,
    privacy_level: Option<PrivacyLevel>,
    other_keywords: Option<String>,
) -> Result<(), error::UpdateWithMetadata> {
    let mut txn = pool.begin().await?;

    let draft_id = sqlx::query!(
        //language=SQL
        r#"
select draft_id from jig join jig_data on jig.draft_id = jig_data.id where jig.id = $1 for update
"#,
        id.0
    )
    .fetch_optional(&mut txn)
    .await?
    .ok_or(error::UpdateWithMetadata::ResourceNotFound)?
    .draft_id;

    // update nullable fields
    if let Some(audio_background) = audio_background {
        sqlx::query!(
            //language=SQL
            r#"
update jig_data
set audio_background = $2
where id = $1 and $2 is distinct from audio_background
            "#,
            draft_id,
            audio_background.map(|it| it as i16),
        )
        .execute(&mut txn)
        .await?;
    }

    // update collection fields, where HashSet<_> maps to an array[] column
    if let Some(audio_effects) = audio_effects {
        sqlx::query!(
            //language=SQL
            r#"
update jig_data
set audio_feedback_positive = $2,
    audio_feedback_negative = $3
where id = $1 and ($2 <> audio_feedback_positive or $3 <> audio_feedback_negative)
            "#,
            draft_id,
            &audio_effects
                .feedback_positive
                .iter()
                .map(|it| *it as i16)
                .collect::<Vec<_>>(),
            &audio_effects
                .feedback_negative
                .iter()
                .map(|it| *it as i16)
                .collect::<Vec<_>>(),
        )
        .execute(&mut txn)
        .await?;
    }

    if let Some(settings) = default_player_settings {
        sqlx::query!(
            //language=SQL
            r#"
update jig_data
set direction = $2,
    display_score = $3,
    track_assessments = $4,
    drag_assist = $5
where id = $1 and
    (($2 is distinct from direction) or
     ($3 is distinct from display_score) or
     ($4 is distinct from track_assessments) or
     ($5 is distinct from drag_assist))
            "#,
            draft_id,
            settings.direction as i16,
            settings.display_score,
            settings.track_assessments,
            settings.drag_assist,
        )
        .execute(&mut txn)
        .await?;
    }

    if let Some(privacy_level) = privacy_level {
        sqlx::query!(
            //language=SQL
            r#"
update jig_data
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
        let translated_description: Option<HashMap<String, String>> = if !description.is_empty() {
            let translate_text: Option<HashMap<String, String>> = match &api_key {
                Some(key) => multi_translation(description, key)
                    .await
                    .context("could not translate text")?,
                None => None,
            };
            translate_text
        } else {
            None
        };

        sqlx::query!(
            r#"
update jig_data
set description = $2,
    translated_description = (case when length($2) <> 0 then $3::jsonb else '{}'::jsonb end),
    updated_at = now()
where id = $1 and $2 is distinct from description"#,
            draft_id,
            description,
            json!(translated_description)
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
update jig_data
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
update jig_data
set display_name     = coalesce($2, display_name),
    language         = coalesce($3, language),
    theme            = coalesce($4, theme)
where id = $1
  and (($2::text is not null and $2 is distinct from display_name) or
       ($3::text is not null and $3 is distinct from language) or
       ($4::smallint is not null and $4 is distinct from theme))
"#,
        draft_id,
        display_name,
        language,
        theme.map(|it| *it as i16),
    )
    .execute(&mut txn)
    .await?;

    if let Some(goals) = goals {
        super::recycle_metadata(&mut txn, "jig_data", draft_id, goals)
            .await
            .map_err(super::meta::handle_metadata_err)?;
    }

    if let Some(categories) = categories {
        super::recycle_metadata(&mut txn, "jig_data", draft_id, categories)
            .await
            .map_err(super::meta::handle_metadata_err)?;
    }

    if let Some(affiliations) = affiliations {
        super::recycle_metadata(&mut txn, "jig_data", draft_id, affiliations)
            .await
            .map_err(super::meta::handle_metadata_err)?;
    }

    if let Some(age_ranges) = age_ranges {
        super::recycle_metadata(&mut txn, "jig_data", draft_id, age_ranges)
            .await
            .map_err(super::meta::handle_metadata_err)?;
    }

    txn.commit().await?;

    Ok(())
}

pub async fn delete(pool: &PgPool, id: JigId) -> Result<(), error::Delete> {
    let mut txn = pool.begin().await?;

    let (draft_id, live_id) = get_draft_and_live_ids(&mut txn, id)
        .await
        .ok_or(error::Delete::ResourceNotFound)?;

    sqlx::query!(
        //language=SQL
        r#"
with del_data as (
    delete from jig_data
        where id is not distinct from $1 or id is not distinct from $2)
delete
from jig
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

pub async fn delete_all_jigs(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<DeleteUserJigs>, error::Delete> {
    let mut txn = pool.begin().await?;

    let jig_ids: Vec<DeleteUserJigs> = get_user_jig_ids(&mut txn, user_id).await?;

    sqlx::query!(
        //language=SQL
        r#"
delete
from jig
where creator_id is not distinct from $1
"#,
        user_id
    )
    .execute(&mut txn)
    .await?;

    txn.commit().await?;

    Ok(jig_ids)
}

// `None` here means do not filter.
pub async fn filtered_count(
    db: &PgPool,
    privacy_level: Option<PrivacyLevel>,
    author_id: Option<Uuid>,
    jig_focus: Option<JigFocus>,
    draft_or_live: Option<DraftOrLive>,
) -> sqlx::Result<u64> {
    sqlx::query!(
        //language=SQL
        r#"
select count(*) as "count!: i64"
from jig
left join jig_admin_data "admin" on admin.jig_id = jig.id
left join jig_data "jig_dt" on jig_dt.id = jig.draft_id
left join jig_data "jig_tt" on jig_tt.id = jig.live_id
where (jig_dt.privacy_level = $1 or $1 is null)
    and (author_id = $2 or $2 is null)
    and (jig_focus = $3 or $3 is null)
    and ((jig_dt.draft_or_live = $4 or $4 is null)
    or (jig_tt.draft_or_live = $4 or $4 is null))
"#,
        privacy_level.map(|it| it as i16),
        author_id,
        jig_focus.map(|it| it as i16),
        draft_or_live.map(|it| it as i16)
    )
    .fetch_one(db)
    .await
    .map(|it| it.count as u64)
}

pub async fn count(db: &PgPool, privacy_level: PrivacyLevel) -> sqlx::Result<u64> {
    sqlx::query!(
        //language=SQL
        r#"
select count(*) as "count!: i64"
from jig_data
inner join jig on jig.live_id = jig_data.id
where (privacy_level = coalesce($1, privacy_level))
and (jig_focus = coalesce($1, jig_focus))
"#,
        privacy_level as i16,
    )
    .fetch_one(db)
    .await
    .map(|it| it.count as u64)
}

pub async fn get_draft_and_live_ids(txn: &mut PgConnection, jig_id: JigId) -> Option<(Uuid, Uuid)> {
    sqlx::query!(
        //language=SQL
        r#"
select draft_id, live_id from jig where id = $1
"#,
        jig_id.0
    )
    .fetch_optional(&mut *txn)
    .await
    .ok()?
    .map(|it| (it.draft_id, it.live_id))
}

pub async fn get_user_jig_ids(
    txn: &mut PgConnection,
    user_id: Uuid,
) -> sqlx::Result<Vec<DeleteUserJigs>> {
    sqlx::query_as!(
        //language=SQL
        DeleteUserJigs,
        r#"
select id as "jig_id!: JigId" from jig where creator_id = $1
"#,
        user_id
    )
    .fetch_all(&mut *txn)
    .await
}

/// Clones a copy of the jig data and modules, preserving the module's stable IDs
pub async fn clone_data(
    txn: &mut PgConnection,
    from_data_id: &Uuid,
    draft_or_live: DraftOrLive,
) -> Result<Uuid, error::JigCloneDraft> {
    println!("here in clone");
    let new_id = sqlx::query!(
        //language=SQL
        r#"
insert into jig_data
(display_name, created_at, updated_at, language, last_synced_at, description, theme, audio_background,
 audio_feedback_negative, audio_feedback_positive, direction, display_score, drag_assist, track_assessments, privacy_level, other_keywords, translated_keywords, translated_description)
select display_name,
       created_at,
       updated_at,
       language,
       last_synced_at,
       description,
       theme,
       audio_background,
       audio_feedback_negative,
       audio_feedback_positive,
       direction,
       display_score,
       drag_assist,
       track_assessments,
       privacy_level,
       other_keywords,
       translated_keywords,
       translated_description::jsonb
from jig_data
where id = $1
returning id
        "#,
        from_data_id,
    )
    .fetch_one(&mut *txn)
    .await?
    .id;

    println!("after in clone");

    update_draft_or_live(txn, new_id, draft_or_live).await?;

    // copy metadata
    sqlx::query!(
        //language=SQL
        r#"
insert into jig_data_module (stable_id, "index", jig_data_id, kind, is_complete, contents)
select stable_id, "index", $2 as "jig_id", kind, is_complete, contents
from jig_data_module
where jig_data_id = $1
        "#,
        from_data_id,
        new_id,
    )
    .execute(&mut *txn)
    .await?;

    sqlx::query!(
        //language=SQL
        r#"
insert into jig_data_additional_resource(jig_data_id, resource_type_id, display_name, resource_content)
select $2, resource_type_id, display_name, resource_content
from jig_data_additional_resource
where jig_data_id = $1
        "#,
        from_data_id,
        new_id,
    )
    .execute(&mut *txn)
    .await?;

    sqlx::query!(
        //language=SQL
        r#"
insert into jig_data_affiliation(jig_data_id, affiliation_id)
select $2, affiliation_id
from jig_data_affiliation
where jig_data_id = $1
        "#,
        from_data_id,
        new_id,
    )
    .execute(&mut *txn)
    .await?;

    sqlx::query!(
        //language=SQL
        r#"
insert into jig_data_age_range(jig_data_id, age_range_id)
select $2, age_range_id
from jig_data_age_range
where jig_data_id = $1
        "#,
        from_data_id,
        new_id,
    )
    .execute(&mut *txn)
    .await?;

    sqlx::query!(
        //language=SQL
        r#"
insert into jig_data_category(jig_data_id, category_id)
select $2, category_id
from jig_data_category
where jig_data_id = $1
        "#,
        from_data_id,
        new_id,
    )
    .execute(&mut *txn)
    .await?;

    sqlx::query!(
        //language=SQL
        r#"
insert into jig_data_goal(jig_data_id, goal_id)
select $2, goal_id
from jig_data_goal
where jig_data_id = $1
        "#,
        from_data_id,
        new_id,
    )
    .execute(&mut *txn)
    .await?;

    // copy modules

    Ok(new_id)
}

pub async fn clone_jig(
    db: &PgPool,
    parent: JigId,
    user_id: Uuid,
) -> Result<JigId, error::JigCloneDraft> {
    let mut txn = db.begin().await?;

    let (draft_id, live_id) = get_draft_and_live_ids(&mut *txn, parent)
        .await
        .ok_or(error::JigCloneDraft::ResourceNotFound)?;

    let new_draft_id = clone_data(&mut txn, &draft_id, DraftOrLive::Draft).await?;
    let new_live_id = clone_data(&mut txn, &live_id, DraftOrLive::Live).await?;

    let new_jig = sqlx::query!(
        //language=SQL
        r#"
insert into jig (creator_id, author_id, parents, live_id, draft_id, published_at, jig_focus)
select creator_id, $2, array_append(parents, $1), $3, $4, published_at, jig_focus
from jig
where id = $1
returning id as "id!: JigId"
"#,
        parent.0,
        user_id,
        new_live_id,
        new_draft_id,
    )
    .fetch_one(&mut txn)
    .await?;

    sqlx::query!(
        // language=SQL
        r#"
insert into jig_play_count (jig_id, play_count)
values ($1, 0)
        "#,
        new_jig.id.0
    )
    .execute(&mut txn)
    .await?;

    txn.commit().await?;

    Ok(new_jig.id)
}

pub async fn get_play_count(db: &PgPool, id: JigId) -> Result<i64, error::NotFound> {
    let play_count = sqlx::query!(
        // language=SQL
        r#"
select play_count from jig_play_count
where jig_id = $1;
            "#,
        id.0,
    )
    .fetch_optional(db)
    .await?
    .ok_or(error::NotFound::ResourceNotFound)?
    .play_count;

    Ok(play_count)
}

pub async fn jig_play(db: &PgPool, id: JigId) -> anyhow::Result<()> {
    let mut txn = db.begin().await?;

    let jig = sqlx::query!(
        // language=SQL
        r#"
select published_at  as "published_at?"
from jig
where id = $1
    "#,
        id.0
    )
    .fetch_one(&mut txn)
    .await?;

    //check if jig has been published and playable
    if jig.published_at == None {
        return Err(anyhow::anyhow!("Jig has not been published"));
    };

    //update Jig play count
    sqlx::query!(
        // language=SQL
        r#"
update jig_play_count
set play_count = play_count + 1
where jig_id = $1;
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
    jig_id: JigId,
    rating: Option<JigRating>,
    blocked: Option<bool>,
    curated: Option<bool>,
) -> Result<(), error::NotFound> {
    let mut txn = pool.begin().await?;

    if let Some(rating) = rating {
        sqlx::query!(
            //language=SQL
            r#"
update jig_admin_data
set rating = coalesce($2, rating)
where jig_id = $1 and $2 is distinct from rating
            "#,
            jig_id.0,
            rating as i16
        )
        .execute(&mut txn)
        .await?;
    }

    if let Some(blocked) = blocked {
        sqlx::query!(
            //language=SQL
            r#"
update jig_admin_data
set blocked = coalesce($2, blocked)
where jig_id = $1 and $2 is distinct from blocked
            "#,
            jig_id.0,
            blocked
        )
        .execute(&mut txn)
        .await?;
    }

    if let Some(curated) = curated {
        sqlx::query!(
            //language=SQL
            r#"
update jig_admin_data
set curated = coalesce($2, curated)
where jig_id = $1 and $2 is distinct from curated
            "#,
            jig_id.0,
            curated
        )
        .execute(&mut txn)
        .await?;
    }

    txn.commit().await?;

    Ok(())
}

pub async fn jig_like(db: &PgPool, user_id: Uuid, id: JigId) -> anyhow::Result<()> {
    let mut txn = db.begin().await?;

    let jig = sqlx::query!(
        r#"
select author_id,
       published_at  as "published_at?"
from jig
where id = $1
    "#,
        id.0
    )
    .fetch_one(&mut txn)
    .await?;

    //check if Jig is published and likeable
    if jig.published_at == None {
        return Err(anyhow::anyhow!("Jig has not been published"));
    };

    // check if current user is the author
    if jig.author_id == Some(user_id) {
        return Err(anyhow::anyhow!("Cannot like your own jig"));
    };

    // checks if user has already liked the jig
    sqlx::query!(
        // language=SQL
        r#"
insert into jig_like(jig_id, user_id)
values ($1, $2)
            "#,
        id.0,
        user_id
    )
    .execute(&mut txn)
    .await
    .map_err(|_| anyhow::anyhow!("Cannot like a jig more than once"))?;

    txn.commit().await?;

    Ok(())
}

pub async fn jig_unlike(db: &PgPool, user_id: Uuid, id: JigId) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
delete from jig_like
where jig_id = $1 and user_id = $2
    "#,
        id.0,
        user_id
    )
    .execute(db)
    .await
    .map_err(|_| anyhow::anyhow!("Must like jig prior to unlike"))?;

    Ok(())
}

pub async fn jig_is_liked(db: &PgPool, user_id: Uuid, id: JigId) -> sqlx::Result<bool> {
    let exists = sqlx::query!(
        r#"
select exists (
    select 1
    from jig_like
    where
        jig_id = $1
        and user_id = $2
) as "exists!"
    "#,
        id.0,
        user_id
    )
    .fetch_one(db)
    .await?
    .exists;

    Ok(exists)
}

/////////
// Auth based on user scope or jig ownership

pub async fn authz_list(
    db: &PgPool,
    user_id: Uuid,
    author_id: Option<Uuid>,
) -> Result<(), error::Auth> {
    let scopes: &[_] = if author_id == Some(user_id) {
        &[
            UserScope::Admin as i16,
            UserScope::AdminJig as i16,
            UserScope::ManageSelfJig as i16,
        ][..]
    } else {
        &[UserScope::Admin as i16, UserScope::AdminJig as i16][..]
    };

    let authed = sqlx::query!(
        r#"
select exists(select 1 from user_scope where user_id = $1 and scope = any($2)) as "authed!"
"#,
        user_id,
        scopes,
    )
    .fetch_one(db)
    .await?
    .authed;

    if !authed {
        return Err(error::Auth::Forbidden);
    }

    Ok(())
}

pub async fn authz(db: &PgPool, user_id: Uuid, jig_id: Option<JigId>) -> Result<(), error::Auth> {
    let authed = match jig_id {
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
    not exists (select 1 from jig where jig.id = $4 and jig.author_id <> $1)
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
    jig_data_id: Uuid,
    draft_or_live: DraftOrLive,
) -> sqlx::Result<()> {
    sqlx::query!(
        //language=SQL
        r#"
update jig_data
set draft_or_live = $2
where id = $1
            "#,
        jig_data_id,
        draft_or_live as i16
    )
    .execute(&mut *conn)
    .await?;

    Ok(())
}
