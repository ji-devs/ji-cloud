use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use shared::domain::{
    category::CategoryId,
    jig::{
        additional_resource::AdditionalResourceId,
        module::{
            body::{cover, ThemeId},
            ModuleId,
        },
        AudioBackground, AudioEffects, AudioFeedbackNegative, AudioFeedbackPositive, Jig, JigId,
        JigPlayerSettings, LiteModule, ModuleKind, PrivacyLevel, TextDirection,
    },
    meta::{AffiliationId, AgeRangeId, GoalId},
    user::UserScope,
};
use sqlx::{postgres::PgDatabaseError, PgConnection, PgPool};
use uuid::Uuid;

use crate::error;

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
    publish_at: Option<DateTime<Utc>>,
    language: &str,
    description: &str,
    default_player_settings: &JigPlayerSettings,
) -> Result<JigId, CreateJigError> {
    let mut transaction = pool.begin().await?;

    let jig = sqlx::query!(
        // language=SQL
        r#"
insert into jig
   (display_name, creator_id, author_id, publish_at, language, description, direction, display_score, track_assessments, drag_assist)
values ($1, $2, $2, $3, $4, $5, $6, $7, $8, $9)
returning id
"#,
        display_name,
        creator_id,
        publish_at,
        language,
        description,
        default_player_settings.direction as i16,
        default_player_settings.display_score,
        default_player_settings.track_assessments,
        default_player_settings.drag_assist,
    )
    .fetch_one(&mut transaction)
    .await?;

    super::recycle_metadata(&mut transaction, "jig", jig.id, goals).await?;
    super::recycle_metadata(&mut transaction, "jig", jig.id, categories).await?;
    super::recycle_metadata(&mut transaction, "jig", jig.id, age_ranges).await?;
    super::recycle_metadata(&mut transaction, "jig", jig.id, affiliations).await?;

    let default_modules = [(
        ModuleKind::Cover,
        serde_json::to_value(cover::ModuleData::default())
            .expect("default cover module failed to serialize while creating jig"),
    )];

    // todo: batch
    for (idx, (kind, contents)) in default_modules.iter().enumerate() {
        sqlx::query!(
            r#"
insert into jig_module (jig_id, "index", kind, contents)
values ($1, $2, $3, $4)"#,
            jig.id,
            idx as i16,
            (*kind) as i16,
            contents
        )
        .execute(&mut transaction)
        .await?;
    }

    // todo add play_count table
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

pub async fn get_by_ids(
    db: &PgPool,
    ids: &[Uuid],
    privacy_level: PrivacyLevel,
    is_draft: bool,
) -> sqlx::Result<Vec<Jig>> {
    let v = sqlx::query!( //language=SQL
r#"
select id                                                                            as "id!: JigId",
       display_name                                                                  as "display_name!",
       creator_id,
       author_id                                                                     as "author_id",
       (select given_name || ' '::text || family_name
        from user_profile
        where user_profile.user_id = author_id)                                      as "author_name",
       publish_at,
       updated_at,
       is_draft                                                                      as "is_draft!",
       privacy_level                                                                 as "privacy_level!: PrivacyLevel",
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
               select row (id, kind)
               from jig_module
               where jig_id = jig.id
               order by "index"
           )                                                                         as "modules!: Vec<(ModuleId, ModuleKind)>",
       array(select row (goal_id) from jig_goal where jig_id = jig.id)               as "goals!: Vec<(GoalId,)>",
       array(select row (category_id) from jig_category where jig_id = jig.id)       as "categories!: Vec<(CategoryId,)>",
       array(select row (affiliation_id) from jig_affiliation where jig_id = jig.id) as "affiliations!: Vec<(AffiliationId,)>",
       array(select row (age_range_id) from jig_age_range where jig_id = jig.id)     as "age_ranges!: Vec<(AgeRangeId,)>",
       array(select row (id) from jig_additional_resource where jig_id = jig.id)     as "additional_resources!: Vec<(AdditionalResourceId,)>"
from jig
         inner join unnest($1::uuid[])
    with ordinality t(id, ord) USING (id)
where is_draft = $2 and privacy_level = $3
order by t.ord
"#,
        ids,
        is_draft,
        privacy_level as i16,
    )
        .fetch_all(db)
        .await?;

    let v = v
        .into_iter()
        .map(|row| Jig {
            id: row.id,
            display_name: row.display_name,
            modules: row
                .modules
                .into_iter()
                .map(|(id, kind)| LiteModule { id, kind })
                .collect(),
            goals: row.goals.into_iter().map(|(goal,)| goal).collect(),
            creator_id: row.creator_id,
            author_id: row.author_id,
            author_name: row.author_name,
            language: row.language,
            categories: row.categories.into_iter().map(|(it,)| it).collect(),
            publish_at: row.publish_at,
            last_edited: row.updated_at,
            is_draft: row.is_draft,
            privacy_level: row.privacy_level,
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
                .additional_resources
                .into_iter()
                .map(|(it,)| it)
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
        })
        .collect();

    Ok(v)
}

pub async fn get(pool: &PgPool, id: JigId, is_live: bool) -> anyhow::Result<Option<Jig>> {
    let jig = sqlx::query!( //language=SQL
        r#"
select id                                                                            as "id: JigId",
       display_name,
       creator_id,
       author_id,
       (select given_name || ' '::text || family_name
        from user_profile
        where user_profile.user_id = author_id)                                      as "author_name",
       publish_at,
       updated_at,
       is_draft                                                                      as "is_draft!",
       privacy_level                                                                 as "privacy_level!: PrivacyLevel",
       language,
       description,
       direction                                                                     as "direction: TextDirection",
       display_score,
       track_assessments,
       drag_assist,
       theme                                                                         as "theme: ThemeId",
       audio_background                                                              as "audio_background!: Option<AudioBackground>",
       array(select row (unnest(audio_feedback_positive)))                           as "audio_feedback_positive!: Vec<(AudioFeedbackPositive,)>",
       array(select row (unnest(audio_feedback_negative)))                           as "audio_feedback_negative!: Vec<(AudioFeedbackNegative,)>",
       array(
               select row (id, kind)
               from jig_module
               where jig_id = $1
               order by "index"
           )                                                                         as "modules!: Vec<(ModuleId, ModuleKind)>",
       array(select row (goal_id) from jig_goal where jig_id = $1)                   as "goals!: Vec<(GoalId,)>",
       array(select row (category_id) from jig_category where jig_id = $1)           as "categories!: Vec<(CategoryId,)>",
       array(select row (affiliation_id) from jig_affiliation where jig_id = jig.id) as "affiliations!: Vec<(AffiliationId,)>",
       array(select row (age_range_id) from jig_age_range where jig_id = jig.id)     as "age_ranges!: Vec<(AgeRangeId,)>",
       array(select row (id) from jig_additional_resource where jig_id = $1)         as "additional_resources!: Vec<(AdditionalResourceId,)>"
from jig
where id = $1"#,
        id.0
    )
        .fetch_optional(pool)
        .await?
        .map(|row| Jig {
            id: row.id,
            display_name: row.display_name,
            language: row.language,
            modules: row
                .modules
                .into_iter()
                .map(|(id, kind)| LiteModule { id, kind })
                .collect(),
            goals: row.goals.into_iter().map(|(it, )| it).collect(),
            categories: row.categories.into_iter().map(|(it, )| it).collect(),
            creator_id: row.creator_id,
            author_id: row.author_id,
            author_name: row.author_name,
            publish_at: row.publish_at,
            last_edited: row.updated_at,
            is_draft: row.is_draft,
            privacy_level: row.privacy_level,
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
                feedback_positive: row.audio_feedback_positive.into_iter().map(|(it,)| it).collect(),
                feedback_negative: row.audio_feedback_negative.into_iter().map(|(it,)| it).collect(),
            },
        });

    Ok(jig)
}

pub async fn update(
    pool: &PgPool,
    id: JigId,
    display_name: Option<&str>,
    author_id: Option<Uuid>,
    goals: Option<&[GoalId]>,
    categories: Option<&[CategoryId]>,
    age_ranges: Option<&[AgeRangeId]>,
    affiliations: Option<&[AffiliationId]>,
    privacy_level: Option<&PrivacyLevel>,
    language: Option<&str>,
    description: Option<&str>,
    default_player_settings: Option<&JigPlayerSettings>,
    theme: Option<&ThemeId>,
    audio_background: Option<&Option<AudioBackground>>,
    audio_effects: Option<&AudioEffects>,
) -> Result<(), error::UpdateWithMetadata> {
    let mut transaction = pool.begin().await?;

    if !sqlx::query!(
        r#"select exists(select 1 from jig where id = $1 for update) as "exists!""#,
        id.0
    )
    .fetch_one(&mut transaction)
    .await?
    .exists
    {
        return Err(error::UpdateWithMetadata::ResourceNotFound);
    }

    // update non-trivial fields, e.g.:
    //  Option<Option<_>>, maps to a nullable column
    //  Option<HashSet<_>>, maps to an array[] column

    if let Some(audio_background) = audio_background {
        sqlx::query!(
            r#"
update jig
set audio_background = $2
where id = $1 and $2 is distinct from audio_background
            "#,
            id.0,
            audio_background.map(|it| it as i16),
        )
        .execute(&mut transaction)
        .await?;
    }

    if let Some(audio_effects) = audio_effects {
        sqlx::query!(
            //language=SQL
            r#"
update jig
set audio_feedback_positive = $2,
    audio_feedback_negative = $3
where id = $1 and ($2 <> audio_feedback_positive or $3 <> audio_feedback_negative)
            "#,
            id.0,
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
        .execute(&mut transaction)
        .await?;
    }

    if let Some(settings) = default_player_settings {
        sqlx::query!(
            //language=SQL
            r#"
update jig 
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
            id.0,
            settings.direction as i16,
            settings.display_score,
            settings.track_assessments,
            settings.drag_assist,
        )
        .execute(&mut transaction)
        .await?;
    }

    // update trivial fields
    sqlx::query!(
        //language=SQL
        r#"
update jig
set display_name     = coalesce($2, display_name),
    author_id        = coalesce($3, author_id),
    language         = coalesce($4, language),
    description      = coalesce($5, description),
    privacy_level    = coalesce($6, privacy_level),
    theme            = coalesce($7, theme)
where id = $1
  and (($2::text is not null and $2 is distinct from display_name) or
       ($3::uuid is not null and $3 is distinct from author_id) or
       ($4::text is not null and $4 is distinct from language) or
       ($5::text is not null and $5 is distinct from description) or
       ($6::smallint is not null and $6 is distinct from privacy_level) or
       ($7::smallint is not null and $7 is distinct from theme))
"#,
        id.0,
        display_name,
        author_id,
        language,
        description,
        privacy_level.map(|it| *it as i16),
        theme.map(|it| *it as i16),
    )
    .execute(&mut transaction)
    .await?;

    if let Some(goals) = goals {
        super::recycle_metadata(&mut transaction, "jig", id.0, goals)
            .await
            .map_err(super::meta::handle_metadata_err)?;
    }

    if let Some(categories) = categories {
        super::recycle_metadata(&mut transaction, "jig", id.0, categories)
            .await
            .map_err(super::meta::handle_metadata_err)?;
    }

    if let Some(affiliations) = affiliations {
        super::recycle_metadata(&mut transaction, "jig", id.0, affiliations)
            .await
            .map_err(super::meta::handle_metadata_err)?;
    }

    if let Some(age_ranges) = age_ranges {
        super::recycle_metadata(&mut transaction, "jig", id.0, age_ranges)
            .await
            .map_err(super::meta::handle_metadata_err)?;
    }

    transaction.commit().await?;

    Ok(())
}

pub async fn delete(pool: &PgPool, id: JigId) -> anyhow::Result<()> {
    sqlx::query!(
        //language=SQL
        r#"
with draft as (
    select draft_id as id from jig_draft_join where live_id = $1
)
delete from jig where id = $1 or id = (select id from draft)
"#,
        id.0
    )
    .execute(pool)
    .await
    .map(drop)
    .map_err(Into::into)
}

pub async fn list(
    pool: &sqlx::Pool<sqlx::Postgres>,
    is_published: Option<bool>,
    author_id: Option<Uuid>,
    page: i32,
) -> sqlx::Result<Vec<Jig>> {
    log::info!("{:?}", author_id);
    sqlx::query!( //language=SQL
        r#"
select id                                                                            as "id: JigId",
       display_name,
       creator_id,
       author_id,
       (select given_name || ' '::text || family_name
        from user_profile
        where user_profile.user_id = author_id)                                      as "author_name",
       publish_at,
       updated_at,
       is_draft                                                                      as "is_draft!",
       privacy_level                                                                 as "privacy_level!: PrivacyLevel",
       language,
       description,
       direction                                                                     as "direction: TextDirection",
       display_score,
       track_assessments,
       drag_assist,
       theme                                                                         as "theme: ThemeId",
       audio_background                                                              as "audio_background!: Option<AudioBackground>",
       array(select row (unnest(audio_feedback_positive)))                           as "audio_feedback_positive!: Vec<(AudioFeedbackPositive,)>",
       array(select row (unnest(audio_feedback_negative)))                           as "audio_feedback_negative!: Vec<(AudioFeedbackNegative,)>",
       array(
               select row (id, kind)
               from jig_module
               where jig_id = jig.id
               order by "index"
           )                                                                         as "modules!: Vec<(ModuleId, ModuleKind)>",
       array(select row (goal_id) from jig_goal where jig_id = jig.id)               as "goals!: Vec<(GoalId,)>",
       array(select row (category_id) from jig_category where jig_id = jig.id)       as "categories!: Vec<(CategoryId,)>",
       array(select row (affiliation_id) from jig_affiliation where jig_id = jig.id) as "affiliations!: Vec<(AffiliationId,)>",
       array(select row (age_range_id) from jig_age_range where jig_id = jig.id)     as "age_ranges!: Vec<(AgeRangeId,)>",
       array(select row (id) from jig_additional_resource where jig_id = jig.id)     as "additional_resources!: Vec<(AdditionalResourceId,)>"
from jig
where (publish_at < now() is not distinct from $1 or $1 is null)
  and (author_id is not distinct from $3 or $3 is null)
  and jig.id not in (select draft_id as id from jig_draft_join)
order by coalesce(updated_at, created_at) desc
limit 20 offset 20 * $2
"#,
        is_published,
        page,
        author_id,
    )
        .fetch(pool)
        .map_ok(|row| Jig {
            id: row.id,
            display_name: row.display_name,
            language: row.language,
            modules: row
                .modules
                .into_iter()
                .map(|(id, kind)| LiteModule { id, kind })
                .collect(),
            goals: row.goals.into_iter().map(|(it, )| it).collect(),
            categories: row.categories.into_iter().map(|(it, )| it).collect(),
            creator_id: row.creator_id,
            author_id: row.author_id,
            author_name: row.author_name,
            publish_at: row.publish_at,
            last_edited: row.updated_at,
            is_draft: row.is_draft,
            privacy_level: row.privacy_level,
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

pub async fn get_draft(db: &PgPool, live_id: JigId) -> Result<JigId, error::JigCloneDraft> {
    let mut txn = db.begin().await?;

    let is_draft = sqlx::query!(
        //language=SQL
        r#"select is_draft as "is_draft!" from jig where id = $1"#,
        live_id.0
    )
    .fetch_optional(&mut txn)
    .await?
    .map(|it| it.is_draft);
    match is_draft {
        None => return Err(error::JigCloneDraft::ResourceNotFound),
        Some(is_draft) => {
            if is_draft {
                return Err(error::JigCloneDraft::UnprocessableEntity);
            }
        }
    }

    let res = sqlx::query!(
        // language=SQL
        r#"select draft_id as "id: JigId" from jig_draft_join where live_id = $1"#,
        live_id.0,
    )
    .fetch_one(&mut txn)
    .await
    .map(|it| it.id);

    txn.commit().await?;

    res.map_err(error::JigCloneDraft::from)
}

pub async fn get_live(db: &PgPool, draft_id: JigId) -> Result<JigId, error::JigCloneDraft> {
    let mut txn = db.begin().await?;

    let is_draft = sqlx::query!(
        //language=SQL
        r#"select is_draft as "is_draft!" from jig where id = $1"#,
        draft_id.0
    )
    .fetch_optional(&mut txn)
    .await?
    .map(|it| it.is_draft);
    match is_draft {
        None => return Err(error::JigCloneDraft::ResourceNotFound),
        Some(is_draft) => {
            if !is_draft {
                return Err(error::JigCloneDraft::UnprocessableEntity);
            }
        }
    }

    let res = sqlx::query!(
        // language=SQL
        r#"select live_id as "id: JigId" from jig_draft_join where draft_id = $1"#,
        draft_id.0,
    )
    .fetch_one(&mut txn)
    .await
    .map(|it| it.id);

    txn.commit().await?;

    res.map_err(error::JigCloneDraft::from)
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
