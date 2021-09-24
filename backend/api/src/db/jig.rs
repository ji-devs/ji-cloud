use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use shared::domain::{
    category::CategoryId,
    jig::{
        additional_resource::AdditionalResourceId,
        module::{
            body::{cover, ThemeId},
            ModuleId, StableModuleId,
        },
        AudioBackground, AudioEffects, AudioFeedbackNegative, AudioFeedbackPositive, DraftOrLive,
        JigData, JigId, JigPlayerSettings, JigResponse, LiteModule, ModuleKind, PrivacyLevel,
        TextDirection,
    },
    meta::{AffiliationId, AgeRangeId, GoalId},
    user::UserScope,
};
use sqlx::{postgres::PgDatabaseError, PgConnection, PgPool};
use uuid::Uuid;

use crate::error;

pub(crate) mod additional_resource;
pub(crate) mod module;
pub(crate) mod player;

pub async fn create(
    pool: &PgPool,
    display_name: &str,
    goals: &[GoalId],
    categories: &[CategoryId],
    age_ranges: &[AgeRangeId],
    affiliations: &[AffiliationId],
    creator_id: Uuid,
    language: &str,
    description: &str,
    default_player_settings: &JigPlayerSettings,
) -> Result<JigId, CreateJigError> {
    let mut transaction = pool.begin().await?;

    let default_modules = [(
        ModuleKind::Cover,
        serde_json::to_value(cover::ModuleData::default())
            .expect("default cover module failed to serialize while creating jig"),
    )];

    let draft_id = create_jig_data(
        &mut transaction,
        DraftOrLive::Draft,
        display_name,
        goals,
        categories,
        age_ranges,
        affiliations,
        language,
        description,
        default_player_settings,
    )
    .await?;

    let mut module_stable_ids = Vec::new();

    for (idx, (kind, contents)) in default_modules.iter().enumerate() {
        let module = sqlx::query!(
            //language=SQL
            r#"
insert into jig_data_module (jig_data_id, "index", kind, contents)
values ($1, $2, $3, $4)
returning stable_id as "stable_id: StableModuleId"
"#,
            draft_id,
            idx as i16,
            (*kind) as i16,
            contents
        )
        .fetch_one(&mut transaction)
        .await?;

        module_stable_ids.push(module.stable_id);
    }

    let live_id = create_jig_data(
        &mut transaction,
        DraftOrLive::Live,
        display_name,
        goals,
        categories,
        age_ranges,
        affiliations,
        language,
        description,
        default_player_settings,
    )
    .await?;

    for (idx, (kind, contents, stable_id)) in default_modules
        .iter()
        .zip(module_stable_ids.iter())
        .map(|it| (&it.0 .0, &it.0 .1, it.1))
        .enumerate()
    {
        let module = sqlx::query!(
            //language=SQL
            r#"
insert into jig_data_module (stable_id, jig_data_id, "index", kind, contents)
values ($1, $2, $3, $4, $5)
"#,
            (*stable_id).0,
            live_id,
            idx as i16,
            (*kind) as i16,
            contents
        )
        .fetch_all(&mut transaction)
        .await?;
    }

    let jig = sqlx::query!(
        //language=SQL
        r#"insert into jig (creator_id, author_id, live_id, draft_id) values ($1, $1, $2, $3) returning id"#,
        creator_id,
        live_id,
        draft_id,
    )
    .fetch_one(&mut transaction)
    .await?;

    sqlx::query!(
        // language=SQL
        r#"
insert into jig_play_count (jig_id, play_count)
values ($1, 0)
        "#,
        jig.id
    )
    .execute(&mut transaction)
    .await?;

    transaction.commit().await?;

    Ok(JigId(jig.id))
}

/// If `default_module_stable_ids` is `Some` is given, then insert the default modules with those stable IDs.
///
/// Otherwise if `None`, then create new stable ids and return them as a part of the response.
pub async fn create_jig_data(
    txn: &mut PgConnection, // FIXME does this work?
    draft_or_live: DraftOrLive,
    display_name: &str,
    goals: &[GoalId],
    categories: &[CategoryId],
    age_ranges: &[AgeRangeId],
    affiliations: &[AffiliationId],
    language: &str,
    description: &str,
    default_player_settings: &JigPlayerSettings,
) -> Result<Uuid, CreateJigError> {
    let jig_data = sqlx::query!(
        // language=SQL
        r#"
insert into jig_data
   (display_name, language, description, direction, display_score, track_assessments, drag_assist)
values ($1, $2, $3, $4, $5, $6, $7)
returning id
"#,
        display_name,
        language,
        description,
        default_player_settings.direction as i16,
        default_player_settings.display_score,
        default_player_settings.track_assessments,
        default_player_settings.drag_assist,
    )
    .fetch_one(&mut *txn)
    .await?;

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
}

impl From<sqlx::Error> for CreateJigError {
    fn from(e: sqlx::Error) -> Self {
        Self::Sqlx(e)
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
    let jig = sqlx::query!( //language=SQL
        r#"
with cte as (
    select id      as "jig_id",
           creator_id,
           author_id,
           case
               when $2 = 0 then jig.draft_id
               when $2 = 1 then jig.live_id
               end as "draft_or_live_id",
           privacy_level,
           published_at
    from jig
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
       language,
       description,
       direction                                           as "direction: TextDirection",
       display_score,
       track_assessments,
       drag_assist,
       theme                                               as "theme: ThemeId",
       audio_background                                    as "audio_background: AudioBackground",
       array(select row (unnest(audio_feedback_positive))) as "audio_feedback_positive!: Vec<(AudioFeedbackPositive,)>",
       array(select row (unnest(audio_feedback_negative))) as "audio_feedback_negative!: Vec<(AudioFeedbackNegative,)>",
       array(
               select row (jig_data_module.id, kind)
               from jig_data_module
               where jig_data_id = cte.draft_or_live_id
               order by "index"
           )                                               as "modules!: Vec<(ModuleId, ModuleKind)>",
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
       array(select row (jig_data_additional_resource.id)
             from jig_data_additional_resource
             where jig_data_id = cte.draft_or_live_id)     as "additional_resources!: Vec<(AdditionalResourceId,)>"
from jig_data
         inner join cte on cte.draft_or_live_id = jig_data.id
"#,
        id.0,
        draft_or_live as i16,
    )
        .fetch_optional(pool)
        .await?
        .map(|row| JigResponse {
            id: row.jig_id,
            privacy_level: row.privacy_level,
            published_at: row.published_at,
            creator_id: row.creator_id,
            author_id: row.author_id,
            author_name: row.author_name,
            jig_data: JigData {
                draft_or_live,
                display_name: row.display_name,
                language: row.language,
                modules: row
                    .modules
                    .into_iter()
                    .map(|(id, kind)| LiteModule { id, kind })
                    .collect(),
                goals: row.goals.into_iter().map(|(it, )| it).collect(),
                categories: row.categories.into_iter().map(|(it, )| it).collect(),
                last_edited: row.updated_at,
                description: row.description,
                default_player_settings: JigPlayerSettings {
                    direction: row.direction,
                    display_score: row.display_score,
                    track_assessments: row.track_assessments,
                    drag_assist: row.drag_assist,
                },
                theme: row.theme,
                age_ranges: row.age_ranges.into_iter().map(|(it, )| it).collect(),
                affiliations: row.affiliations.into_iter().map(|(it, )| it).collect(),
                additional_resources: row.additional_resources.into_iter().map(|(it, )| it).collect(),
                audio_background: row.audio_background,
                audio_effects: AudioEffects {
                    feedback_positive: row.audio_feedback_positive.into_iter().map(|(it, )| it).collect(),
                    feedback_negative: row.audio_feedback_negative.into_iter().map(|(it, )| it).collect(),
                },
            },
        });

    Ok(jig)
}

pub async fn get_by_ids(
    db: &PgPool,
    ids: &[Uuid],
    privacy_level: PrivacyLevel,
    draft_or_live: DraftOrLive,
) -> sqlx::Result<Vec<JigResponse>> {
    let mut txn = db.begin().await?;

    let jig = sqlx::query!(
        //language=SQL
        r#"
select id                                                                            as "id!: JigId",
       creator_id,
       author_id                                                                     as "author_id",
       (select given_name || ' '::text || family_name
        from user_profile
        where user_profile.user_id = author_id)                                      as "author_name",
       privacy_level as "privacy_level!: PrivacyLevel",
       live_id                                                                       as "live_id!",
       draft_id                                                                      as "draft_id!",
       published_at
from jig
         inner join unnest($1::uuid[])
    with ordinality t(id, ord) using (id)
order by t.ord
    "#,
        ids
    )
    .fetch_all(&mut txn)
    .await?;

    let jig_data_ids: Vec<Uuid> = match draft_or_live {
        DraftOrLive::Draft => jig.iter().map(|it| it.draft_id).collect(),
        DraftOrLive::Live => jig.iter().map(|it| it.live_id).collect(),
    };

    let jig_data = sqlx::query!( //language=SQL
r#"
select id,
       display_name                                                                  as "display_name!",
       updated_at,
       language                                                                      as "language!",
       description                                                                   as "description!",
       direction                                                                     as "direction!: TextDirection",
       display_score                                                                 as "display_score!",
       track_assessments                                                             as "track_assessments!",
       drag_assist                                                                   as "drag_assist!",
       theme                                                                         as "theme!: ThemeId",
       audio_background                                                              as "audio_background!: Option<AudioBackground>",
       array(select row (unnest(audio_feedback_positive)))                           as "audio_feedback_positive!: Vec<(AudioFeedbackPositive,)>",
       array(select row (unnest(audio_feedback_negative)))                           as "audio_feedback_negative!: Vec<(AudioFeedbackNegative,)>",
       array(
               select row (jig_data_module.id, kind)
               from jig_data_module
               where jig_data_id = jig_data.id
               order by "index"
           )                                               as "modules!: Vec<(ModuleId, ModuleKind)>",
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
       array(select row (jig_data_additional_resource.id)
             from jig_data_additional_resource
             where jig_data_id = jig_data.id)     as "additional_resources!: Vec<(AdditionalResourceId,)>"
from jig_data
         inner join unnest($1::uuid[])
    with ordinality t(id, ord) using (id)
order by t.ord
"#,
        &jig_data_ids,
    )
        .fetch_all(&mut txn).await?;

    let v = jig
        .into_iter()
        .zip(jig_data.into_iter())
        .map(|(jig_row, jig_data_row)| JigResponse {
            id: jig_row.id,
            privacy_level: jig_row.privacy_level,
            published_at: jig_row.published_at,
            creator_id: jig_row.creator_id,
            author_id: jig_row.author_id,
            author_name: jig_row.author_name,
            jig_data: JigData {
                draft_or_live,
                display_name: jig_data_row.display_name,
                language: jig_data_row.language,
                modules: jig_data_row
                    .modules
                    .into_iter()
                    .map(|(id, kind)| LiteModule { id, kind })
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
                    .additional_resources
                    .into_iter()
                    .map(|(it,)| it)
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
            },
        })
        .collect();

    txn.rollback().await?;

    Ok(v)
}

pub async fn update(
    pool: &PgPool,
    jig_id: JigId,
    privacy_level: PrivacyLevel,
) -> Result<(), error::UpdateWithMetadata> {
    let mut txn = pool.begin().await?;

    sqlx::query!(
        //language=SQL
        r#"
update jig 
set privacy_level = $2
where id = $1
    "#,
        jig_id.0,
        privacy_level as i16,
    )
    .execute(&mut txn)
    .await?;

    txn.commit().await?;

    Ok(())
}

pub async fn update_draft(
    pool: &PgPool,
    id: JigId,
    display_name: Option<&str>,
    author_id: Option<Uuid>,
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
) -> Result<(), error::UpdateWithMetadata> {
    let mut txn = pool.begin().await?;

    let draft_id = sqlx::query!(
        //language=SQL
        r#"
select draft_id from jig where id = $1
"#,
        id.0
    )
    .fetch_optional(&mut txn)
    .await?
    .ok_or(error::UpdateWithMetadata::ResourceNotFound)?
    .draft_id;

    sqlx::query!(
        //language=SQL
        r#"select id from jig_data where id = $1 for update"#,
        draft_id
    )
    .execute(&mut txn)
    .await?;

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

    // update trivial, not null fields
    sqlx::query!(
        //language=SQL
        r#"
update jig_data
set display_name     = coalesce($2, display_name),
    language         = coalesce($3, language),
    description      = coalesce($4, description),
    theme            = coalesce($5, theme)
where id = $1
  and (($2::text is not null and $2 is distinct from display_name) or
       ($3::text is not null and $3 is distinct from language) or
       ($4::text is not null and $4 is distinct from description) or
       ($5::smallint is not null and $5 is distinct from theme))
"#,
        draft_id,
        display_name,
        language,
        description,
        theme.map(|it| *it as i16),
    )
    .execute(&mut txn)
    .await?;

    if let Some(goals) = goals {
        super::recycle_metadata(&mut txn, "jig_data", id.0, goals)
            .await
            .map_err(super::meta::handle_metadata_err)?;
    }

    if let Some(categories) = categories {
        super::recycle_metadata(&mut txn, "jig_data", id.0, categories)
            .await
            .map_err(super::meta::handle_metadata_err)?;
    }

    if let Some(affiliations) = affiliations {
        super::recycle_metadata(&mut txn, "jig_data", id.0, affiliations)
            .await
            .map_err(super::meta::handle_metadata_err)?;
    }

    if let Some(age_ranges) = age_ranges {
        super::recycle_metadata(&mut txn, "jig_data", id.0, age_ranges)
            .await
            .map_err(super::meta::handle_metadata_err)?;
    }

    txn.commit().await?;

    Ok(())
}

pub async fn delete(pool: &PgPool, id: JigId) -> Result<(), error::Delete> {
    let mut txn = pool.begin().await?;

    let (draft_id, live_id) = sqlx::query!(
        //language=SQL
        r#"
select draft_id, live_id from jig where id = $1
"#,
        id.0
    )
    .fetch_optional(&mut txn)
    .await?
    .map(|it| (it.draft_id, it.live_id))
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

pub async fn browse(
    pool: &sqlx::Pool<sqlx::Postgres>,
    author_id: Option<Uuid>,
    page: i32,
) -> sqlx::Result<Vec<JigResponse>> {
    sqlx::query!( //language=SQL
        r#"
select jig.id                                              as "jig_id: JigId",
       privacy_level                                       as "privacy_level: PrivacyLevel",
       creator_id,
       author_id,
       (select given_name || ' '::text || family_name
        from user_profile
        where user_profile.user_id = author_id)            as "author_name",
       published_at,
       display_name                                        as "display_name!",
       updated_at,
       language                                            as "language!",
       description                                         as "description!",
       direction                                           as "direction!: TextDirection",
       display_score                                       as "display_score!",
       track_assessments                                   as "track_assessments!",
       drag_assist                                         as "drag_assist!",
       theme                                               as "theme!: ThemeId",
       audio_background                                    as "audio_background!: Option<AudioBackground>",
       array(select row (unnest(audio_feedback_positive))) as "audio_feedback_positive!: Vec<(AudioFeedbackPositive,)>",
       array(select row (unnest(audio_feedback_negative))) as "audio_feedback_negative!: Vec<(AudioFeedbackNegative,)>",
       array(
               select row (jig_data_module.id, kind)
               from jig_data_module
               where jig_data_id = jig_data.id
               order by "index"
           )                                               as "modules!: Vec<(ModuleId, ModuleKind)>",
       array(select row (goal_id)
             from jig_data_goal
             where jig_data_id = jig_data.id)              as "goals!: Vec<(GoalId,)>",
       array(select row (category_id)
             from jig_data_category
             where jig_data_id = jig_data.id)              as "categories!: Vec<(CategoryId,)>",
       array(select row (affiliation_id)
             from jig_data_affiliation
             where jig_data_id = jig_data.id)              as "affiliations!: Vec<(AffiliationId,)>",
       array(select row (age_range_id)
             from jig_data_age_range
             where jig_data_id = jig_data.id)              as "age_ranges!: Vec<(AgeRangeId,)>",
       array(select row (jig_data_additional_resource.id)
             from jig_data_additional_resource
             where jig_data_id = jig_data.id)              as "additional_resources!: Vec<(AdditionalResourceId,)>"
from jig_data
         inner join jig on jig_data.id = jig.draft_id
where (author_id is not distinct from $2 or $2 is null)
order by coalesce(updated_at, created_at) desc
limit 20 offset 20 * $1
"#,
        page,
        author_id,
    )
        .fetch(pool)
        .map_ok(|row| JigResponse {
            id: row.jig_id,
            privacy_level: row.privacy_level,
            published_at: row.published_at,
            creator_id: row.creator_id,
            author_id: row.author_id,
            author_name: row.author_name,
            jig_data: JigData {
                draft_or_live: DraftOrLive::Draft,
                display_name: row.display_name,
                language: row.language,
                modules: row
                    .modules
                    .into_iter()
                    .map(|(id, kind)| LiteModule { id, kind })
                    .collect(),
                goals: row.goals.into_iter().map(|(it, )| it).collect(),
                categories: row.categories.into_iter().map(|(it, )| it).collect(),
                last_edited: row.updated_at,
                description: row.description,
                default_player_settings: JigPlayerSettings {
                    direction: row.direction,
                    display_score: row.display_score,
                    track_assessments: row.track_assessments,
                    drag_assist: row.drag_assist,
                },
                theme: row.theme,
                age_ranges: row.age_ranges.into_iter().map(|(it, )| it).collect(),
                affiliations: row.affiliations.into_iter().map(|(it, )| it).collect(),
                additional_resources: row.additional_resources.into_iter().map(|(it, )| it).collect(),
                audio_background: row.audio_background,
                audio_effects: AudioEffects {
                    feedback_positive: row.audio_feedback_positive.into_iter().map(|(it, )| it).collect(),
                    feedback_negative: row.audio_feedback_negative.into_iter().map(|(it, )| it).collect(),
                },
            },
        })
        .try_collect()
        .await
}

/// `None` here means do not filter.
pub async fn filtered_count(
    db: &PgPool,
    is_draft: Option<bool>,
    privacy_level: Option<PrivacyLevel>,
    author_id: Option<Uuid>,
) -> sqlx::Result<u64> {
    sqlx::query!(
        //language=SQL
        r#"
select count(*) as "count!: i64"
from jig
where
    (is_draft is not distinct from $1 or $1 is null)
    and (privacy_level is not distinct from $2 or $2 is null)
    and (author_id is not distinct from $3 or $3 is null)
"#,
        is_draft,
        privacy_level.map(|it| it as i16),
        author_id,
    )
    .fetch_one(db)
    .await
    .map(|it| it.count as u64)
}

// FIXME make a struct for named live, draft jig?
pub async fn clone_jig_and_draft(
    db: &PgPool,
    parent: JigId,
    user_id: Uuid,
) -> Result<JigId, error::JigCloneDraft> {
    let mut txn = db.begin().await?;

    let new_live = clone_one(&mut txn, &parent, None, &user_id, false, true).await?;

    let new_draft = clone_one(&mut txn, &parent, None, &user_id, true, true).await?;

    sqlx::query!(
        //language=SQL
        r#"
insert into jig_draft_join (live_id, draft_id)
values ($1, $2)
"#,
        new_live.0,
        new_draft.0,
    )
    .execute(&mut txn)
    .await?;

    txn.commit().await?;

    Ok(new_draft)
}

/// Clones a copy of the jig data and modules, preserving the module's stable IDs
pub async fn clone_data(
    txn: &mut PgConnection,
    parent_data_id: &Uuid,
    target_data_id: &Option<Uuid>,
    user_id: &Uuid,
    target_type: DraftOrLive,
) -> Result<(), error::JigCloneDraft> {
    Ok(())
}

/// Clones a single jig. This should *only* be used with live jigs to avoid invalidating the
/// `parents` chain.
///
/// # Arguments
/// * `target_id` - if `None`, then generate a new ID to clone the
pub async fn clone_one(
    txn: &mut PgConnection,
    parent_id: &JigId,
    target_id: Option<JigId>,
    user_id: &Uuid,
    target_is_draft: bool,
    append_parent: bool,
) -> Result<JigId, error::JigCloneDraft> {
    let target_id = match target_id {
        Some(id) => id,
        None => {
            sqlx::query!(r#"select uuid_generate_v1mc() as "id!: JigId""#)
                .fetch_one(&mut *txn)
                .await?
                .id
        }
    };

    sqlx::query!(
        // language=SQL
        r#"
insert into jig (id, display_name, parents, creator_id, author_id, language, description, direction, display_score,
                 track_assessments, drag_assist, theme, audio_background, audio_feedback_positive,
                 audio_feedback_negative, is_draft, privacy_level)
select $2,
       display_name,
       array_append(parents, $1),
       $3 as creator_id,
       $3 as author_id,
       language,
       description,
       direction,
       display_score,
       track_assessments,
       drag_assist,
       theme,
       audio_background,
       audio_feedback_positive,
       audio_feedback_negative,
       $4, 
       $5
from jig
where id = $1
        "#,
        parent_id.0,
        target_id.0,
        user_id,
        target_is_draft,
        PrivacyLevel::Unlisted as i16,
    )
    .execute(&mut *txn)
    .await
    .map_err(|err| match err {
        sqlx::Error::Database(err)
        if err.downcast_ref::<PgDatabaseError>().constraint()
            == Some("jig_pkey") =>
            {
                error::JigCloneDraft::Conflict
            }
        err => error::JigCloneDraft::InternalServerError(err.into()),
    })?;

    sqlx::query!(
        //language=SQL
        r#"
update jig
set parents = (
    select case
               when $3 = true then array_append(parents, null)
               when $3 = false then parents
               end

    from jig
    where id = $1
)
where id = $2
        "#,
        parent_id.0,
        target_id.0,
        append_parent,
    )
    .execute(&mut *txn)
    .await?;

    sqlx::query!(
        //language=SQL
        r#"
insert into jig_module ("index", jig_id, kind, contents)
select "index", $2 as "jig_id", kind, contents
from jig_module where jig_id = $1
        "#,
        parent_id.0,
        target_id.0
    )
    .execute(&mut *txn)
    .await?;

    sqlx::query!(
        //language=SQL
        r#"
insert into jig_affiliation(jig_id, affiliation_id)
select $2, affiliation_id from jig_affiliation where jig_id = $1
        "#,
        parent_id.0,
        target_id.0,
    )
    .execute(&mut *txn)
    .await?;

    sqlx::query!(
        //language=SQL
        r#"
insert into jig_category(jig_id, category_id)
select $2, category_id from jig_category where jig_id = $1
        "#,
        parent_id.0,
        target_id.0,
    )
    .execute(&mut *txn)
    .await?;

    sqlx::query!(
        //language=SQL
        r#"
insert into jig_goal(jig_id, goal_id)
select $2, goal_id from jig_goal where jig_id = $1
        "#,
        parent_id.0,
        target_id.0,
    )
    .execute(&mut *txn)
    .await?;

    sqlx::query!(
        //language=SQL
        r#"
insert into jig_age_range(jig_id, age_range_id)
select $2, age_range_id from jig_age_range where jig_id = $1
        "#,
        parent_id.0,
        target_id.0,
    )
    .execute(&mut *txn)
    .await?;

    sqlx::query!(
        //language=SQL
        r#"
insert into jig_additional_resource(jig_id, url)
select $2, url from jig_additional_resource where jig_id = $1
        "#,
        parent_id.0,
        target_id.0,
    )
    .execute(&mut *txn)
    .await?;

    Ok(target_id)
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
