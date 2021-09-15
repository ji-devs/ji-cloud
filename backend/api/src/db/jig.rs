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
        JigPlayerSettings, LiteModule, ModuleKind, TextDirection,
    },
    meta::{AffiliationId, AgeRangeId, GoalId},
    user::UserScope,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error;

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

pub async fn get_by_ids(db: &PgPool, ids: &[Uuid]) -> sqlx::Result<Vec<Jig>> {
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
       language                                                                      as "language!",
       description                                                                   as "description!",
       is_public                                                                     as "is_public!",
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
order by t.ord

"#, ids)
        .fetch_all(db).await?;

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
            description: row.description,
            last_edited: row.updated_at,
            is_public: row.is_public,
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

pub async fn get(pool: &PgPool, id: JigId) -> anyhow::Result<Option<Jig>> {
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
       language,
       description,
       is_public,
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
            description: row.description,
            last_edited: row.updated_at,
            is_public: row.is_public,
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
    publish_at: Option<Option<DateTime<Utc>>>,
    language: Option<&str>,
    description: Option<&str>,
    is_public: Option<&bool>,
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
    if let Some(publish_at) = publish_at {
        sqlx::query!(
            r#"
update jig
set publish_at = $2
where id = $1 and $2 is distinct from publish_at"#,
            id.0,
            publish_at
        )
        .execute(&mut transaction)
        .await?;
    }

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

    log::info!("{:?}", audio_effects);
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
    is_public        = coalesce($6, is_public),
    theme            = coalesce($7, theme)
where id = $1
  and (($2::text is not null and $2 is distinct from display_name) or
       ($3::uuid is not null and $3 is distinct from author_id) or
       ($4::text is not null and $4 is distinct from language) or
       ($5::text is not null and $5 is distinct from description) or
       ($6::bool is not null and $6 is distinct from is_public) or
       ($7::smallint is not null and $7 is distinct from theme))
"#,
        id.0,
        display_name,
        author_id,
        language,
        description,
        is_public,
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
       language,
       description,
       is_public,
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
            description: row.description,
            last_edited: row.updated_at,
            is_public: row.is_public,
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

pub async fn filtered_count(
    db: &PgPool,
    is_published: Option<bool>,
    is_public: Option<bool>,
    author_id: Option<Uuid>,
) -> sqlx::Result<u64> {
    sqlx::query!(
        //language=SQL
        r#"
select count(*) as "count!: i64"
from jig
where
    (publish_at < now() is not distinct from $1 or $1 is null)
    and (is_public is not distinct from $2 or $2 is null)
    and (author_id is not distinct from $3 or $3 is null)
    and id not in (select draft_id as id from jig_draft_join) 
"#,
        is_published,
        is_public,
        author_id,
    )
    .fetch_one(db)
    .await
    .map(|it| it.count as u64)
}

pub async fn clone(
    db: &PgPool,
    parent: JigId,
    user_id: Uuid,
) -> Result<JigId, error::JigCloneDraft> {
    let mut txn = db.begin().await?;

    let is_draft = sqlx::query!(
        r#"select exists(select 1 from jig_draft_join where draft_id = $1) as "exists!""#,
        parent.0
    )
    .fetch_one(&mut txn)
    .await?
    .exists;
    if is_draft {
        txn.commit().await?;
        return Err(error::JigCloneDraft::IsDraft);
    }

    let new_id = sqlx::query!(
        //language=SQL
        r#"
insert into jig (display_name, parents, creator_id, author_id, language, description, direction, display_score,
                 track_assessments, drag_assist, theme, audio_background, audio_feedback_positive,
                 audio_feedback_negative)
select display_name,
       array_append(parents, id),
       $2 as creator_id,
       $2 as author_id,
       language,
       description,
       direction,
       display_score,
       track_assessments,
       drag_assist,
       theme,
       audio_background,
       audio_feedback_positive,
       audio_feedback_negative
from jig
where id = $1
returning id as "id: JigId"
        "#,
        parent.0,
        user_id
    )
    .fetch_optional(&mut txn)
    .await?
    .ok_or(error::JigCloneDraft::ResourceNotFound)?
    .id.0;

    sqlx::query!(
        r#"
insert into jig_module ("index", jig_id, kind, contents)
select "index", $2 as "jig_id", kind, contents
from jig_module where jig_id = $1
"#,
        parent.0,
        new_id
    )
    .execute(&mut txn)
    .await?;

    sqlx::query!(
        r#"
insert into jig_affiliation(jig_id, affiliation_id)
select $1, affiliation_id from jig_affiliation where jig_id = $2
"#,
        new_id,
        parent.0
    )
    .execute(&mut txn)
    .await?;

    sqlx::query!(
        r#"
insert into jig_category(jig_id, category_id)
select $1, category_id from jig_category where jig_id = $2
"#,
        new_id,
        parent.0
    )
    .execute(&mut txn)
    .await?;

    sqlx::query!(
        r#"
insert into jig_goal(jig_id, goal_id)
select $1, goal_id from jig_goal where jig_id = $2
"#,
        new_id,
        parent.0
    )
    .execute(&mut txn)
    .await?;

    sqlx::query!(
        r#"
insert into jig_age_range(jig_id, age_range_id)
select $1, age_range_id from jig_age_range where jig_id = $2
"#,
        new_id,
        parent.0
    )
    .execute(&mut txn)
    .await?;

    sqlx::query!(
        r#"
insert into jig_additional_resource(jig_id, url)
select $1, url from jig_additional_resource where jig_id = $2
        "#,
        new_id,
        parent.0
    )
    .execute(&mut txn)
    .await?;

    txn.commit().await?;

    Ok(JigId(new_id))
}

pub async fn create_draft(db: &PgPool, live_id: JigId) -> Result<JigId, error::JigCloneDraft> {
    let mut txn = db.begin().await?;

    let exists_draft = sqlx::query!(
        r#"select exists(select 1 from jig_draft_join where live_id = $1) as "exists!""#,
        live_id.0
    )
    .fetch_one(&mut txn)
    .await?
    .exists;
    if exists_draft {
        txn.commit().await?;
        return Err(error::JigCloneDraft::Conflict);
    }

    let is_draft = sqlx::query!(
        r#"select exists(select 1 from jig_draft_join where draft_id = $1) as "exists!""#,
        live_id.0
    )
    .fetch_one(&mut txn)
    .await?
    .exists;
    if is_draft {
        txn.commit().await?;
        return Err(error::JigCloneDraft::IsDraft);
    }

    let draft_id = sqlx::query!( //language=SQL
        r#"
insert into jig (display_name, parents, creator_id, author_id, language, description, publish_at, is_public,
                 direction, display_score, track_assessments, drag_assist,theme, audio_background,
                 audio_feedback_positive, audio_feedback_negative)
select display_name,
       parents,
       creator_id,
       author_id,
       language,
       description,
       $2,
       false,
       direction,
       display_score,
       track_assessments,
       drag_assist,
       theme,
       audio_background,
       audio_feedback_positive,
       audio_feedback_negative       
from jig
where id = $1
returning id as "id: JigId"
        "#,
        live_id.0,
        chrono::MAX_DATETIME,
    )
    .fetch_optional(&mut txn)
    .await?
    .ok_or(error::JigCloneDraft::ResourceNotFound)?
    .id.0;

    sqlx::query!(
        r#"
insert into jig_draft_join (draft_id, live_id)
values ($1, $2)
        "#,
        draft_id,
        live_id.0
    )
    .execute(&mut txn)
    .await?;

    sqlx::query!(
        r#"
insert into jig_module ("index", jig_id, kind, contents)
select "index", $1 as "jig_id", kind, contents
from jig_module where jig_id = $2
"#,
        draft_id,
        live_id.0
    )
    .execute(&mut txn)
    .await?;

    sqlx::query!(
        r#"
insert into jig_affiliation(jig_id, affiliation_id)
select $1, affiliation_id from jig_affiliation where jig_id = $2
"#,
        draft_id,
        live_id.0
    )
    .execute(&mut txn)
    .await?;

    sqlx::query!(
        r#"
insert into jig_category(jig_id, category_id)
select $1, category_id from jig_category where jig_id = $2
"#,
        draft_id,
        live_id.0
    )
    .execute(&mut txn)
    .await?;

    sqlx::query!(
        r#"
insert into jig_goal(jig_id, goal_id)
select $1, goal_id from jig_goal where jig_id = $2
"#,
        draft_id,
        live_id.0
    )
    .execute(&mut txn)
    .await?;

    sqlx::query!(
        r#"
insert into jig_age_range(jig_id, age_range_id)
select $1, age_range_id from jig_age_range where jig_id = $2
"#,
        draft_id,
        live_id.0
    )
    .execute(&mut txn)
    .await?;

    sqlx::query!(
        r#"
insert into jig_additional_resource(jig_id, url)
select $1, url from jig_additional_resource where jig_id = $2
        "#,
        draft_id,
        live_id.0
    )
    .execute(&mut txn)
    .await?;

    txn.commit().await?;

    Ok(JigId(draft_id))
}

pub async fn get_draft(db: &PgPool, live_id: JigId) -> Result<JigId, error::JigCloneDraft> {
    let is_draft = sqlx::query!(
        r#"select exists(select 1 from jig_draft_join where draft_id = $1) as "exists!""#,
        live_id.0
    )
    .fetch_one(db)
    .await?
    .exists;
    if is_draft {
        return Err(error::JigCloneDraft::IsDraft);
    }

    sqlx::query!(
        r#"
select draft_id as "id: JigId" from jig_draft_join where live_id = $1
        "#,
        live_id.0,
    )
    .fetch_optional(db)
    .await?
    .map(|it| it.id)
    .ok_or(error::JigCloneDraft::ResourceNotFound)
}

pub async fn publish_draft_to_live(
    db: &PgPool,
    live_id: JigId,
) -> Result<(), error::JigCloneDraft> {
    let draft_id = get_draft(db, live_id).await?;

    let mut txn = db.begin().await?;

    // delete live from database
    let res = sqlx::query!(
        r#"
delete from jig where id = $1 returning publish_at as "publish_at: DateTime<Utc>", is_public as "is_public: bool"
        "#,
        live_id.0
    )
    .fetch_one(&mut txn)
    .await?;

    // update draft ids to previous live id
    sqlx::query!(
        r#"
with module as (
         update jig_module set jig_id = $1 where jig_id = $4
     ),
     affiliation as (
         update jig_affiliation set jig_id = $1 where jig_id = $4
     ),
     category as (
         update jig_category set jig_id = $1 where jig_id = $4
     ),
     goal as (
         update jig_goal set jig_id = $1 where jig_id = $4
     ),
     age_range as (
         update jig_age_range set jig_id = $1 where jig_id = $4
     ),
     additional_resource as (
         update jig_additional_resource set jig_id = $1 where jig_id = $4
     )
update jig
set id = $1, publish_at = $2, is_public = $3
where id = $4
"#,
        live_id.0,
        res.publish_at,
        res.is_public,
        draft_id.0,
    )
    .execute(&mut txn)
    .await?;

    txn.commit().await?;

    Ok(())
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
// Auth based on user scope or jig association

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
