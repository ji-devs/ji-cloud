use crate::error;
use serde_json::value::Value;
use shared::domain::{
    additional_resource::{AdditionalResource, AdditionalResourceId as AddId, ResourceContent},
    asset::{DraftOrLive, PrivacyLevel},
    badge::BadgeId,
    category::CategoryId,
    course::{CourseData, CourseId, CourseResponse},
    image::ImageId,
    jig::{
        AudioBackground, AudioEffects, AudioFeedbackNegative, AudioFeedbackPositive, JigAdminData,
        JigData, JigFocus, JigId, JigPlayerSettings, JigRating, JigResponse, TextDirection,
    },
    meta::{AffiliationId, AgeRangeId, ResourceTypeId as TypeId},
    module::{body::ThemeId, LiteModule, ModuleId, ModuleKind},
    user::public_user::PublicUser,
};
use sqlx::{types::Json, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

pub async fn get(pool: &PgPool, user_id: Uuid) -> anyhow::Result<PublicUser, error::NotFound> {
    let mut txn = pool.begin().await?;

    let res = sqlx::query_as!(
        PublicUser,
        r#"
    select  user_id as "id",
            username,
            given_name,
            family_name,
            bio,
            profile_image_id       as "profile_image?: ImageId",
            (select language from user_profile where user_profile.user_id = "user".id and language_public is true)      as "language?",
            (select organization from user_profile where user_profile.user_id = "user".id and organization_public is true)  as "organization?", 
            (select array(select language from user_profile where user_profile.user_id = "user".id and persona_public is true))      as "persona!:Vec<String>", 
            (select location from user_profile where user_profile.user_id = "user".id and location_public is true)      as "location?", 
            array(select badge.id 
                from badge_member bm 
                inner join badge on bm.id = badge.id 
                where bm.user_id = "user".id
            ) as "badges!: Vec<BadgeId>"
        from "user"
            inner join user_profile on "user".id = user_profile.user_id
        where id = $1
        "#,
        user_id,
    )
    .fetch_one(&mut txn)
    .await?;

    txn.rollback().await?;

    Ok(res)
}

pub async fn browse_users(
    pool: &PgPool,
    page: u32,
    page_limit: u64,
) -> anyhow::Result<Vec<PublicUser>> {
    let mut txn = pool.begin().await?;

    let user_data = sqlx::query!(
        r#"
        select  user_id                as "id!",
                username               as "username!",
                given_name             as "given_name!",
                family_name            as "family_name!",
                bio                    as "bio!",
                profile_image_id       as "profile_image?: ImageId",
                (select language from user_profile where user_profile.user_id = "user".id and language_public is true)      as "language?",
                (select organization from user_profile where user_profile.user_id = "user".id and organization_public is true)  as "organization?", 
                (select array(select persona from user_profile where user_profile.user_id = "user".id and persona_public is true))      as "persona!: Vec<String>", 
                (select location from user_profile where user_profile.user_id = "user".id and location_public is true)      as "location?", 
                array(select badge.id 
                    from badge_member bm 
                    inner join badge on bm.id = badge.id 
                    where bm.user_id = "user".id
                ) as "badges!: Vec<BadgeId>"
            from "user"
            inner join user_profile on "user".id = user_profile.user_id
            offset $1
            limit $2
            "#,
            page as i32,
            page_limit as i32,
        )
            .fetch_all(&mut txn)
            .await
            .map_err(|_| anyhow::anyhow!("failed to fetch users"))?;

    let res: Vec<_> = user_data
        .into_iter()
        .map(|row| PublicUser {
            id: row.id,
            username: row.username,
            given_name: row.given_name,
            family_name: row.family_name,
            bio: row.bio,
            profile_image: row.profile_image,
            language: row.language,
            organization: row.organization,
            persona: row.persona,
            location: row.location,
            badges: row.badges,
        })
        .collect();

    txn.rollback().await?;

    Ok(res)
}

pub async fn browse_user_jigs(
    pool: &PgPool,
    user_id: Uuid,
    page: u32,
    page_limit: u64,
) -> anyhow::Result<Vec<JigResponse>> {
    let mut txn = pool.begin().await?;

    let jig_data = sqlx::query!(
        r#"
        with cte as (
            select array(select jd.id as "id!"
            from jig_data "jd"
                  left join jig on live_id = jd.id and jd.last_synced_at is not null
                  left join jig_admin_data "admin" on admin.jig_id = jig.id
                  left join jig_data_additional_resource "resource" on jd.id = resource.jig_data_id
            where author_id = $1
            order by coalesce(updated_at, created_at) desc, jig_id) as id
        ),
        cte1 as (
            select * from unnest((select distinct id from cte)) with ordinality t(id
           , ord) order by ord
        )
        select jig.id                                              as "jig_id: JigId",
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
           (
                    select row(jig_data_module.id, kind, is_complete) 
                    from jig_data_module                
                    where jig_data_id = jig_data.id and "index" = 0
                    order by "index"
           )                                                   as "cover?: (ModuleId, ModuleKind, bool)",
            array(
                    select row (jig_data_module.id, kind, is_complete)
                    from jig_data_module
                    where jig_data_id = jig_data.id and "index" <> 0
                    order by "index"
           )                                               as "modules!: Vec<(ModuleId, ModuleKind, bool)>",
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
        from cte1
        left join jig_data on cte1.id = jig_data.id
        inner join jig on 
                jig_data.id = jig.live_id
                and (last_synced_at is not null
                or jig.published_at is not null)
        left join jig_admin_data "admin" on admin.jig_id = jig.id
        where ord > (1 * $2 * $3)
        order by ord asc
        limit $3
        "#,
            user_id,
            page as i32,
            page_limit as i32,
        )
            .fetch_all(&mut txn)
            .await?;

    let res: Vec<_> = jig_data
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
                cover: jig_data_row
                    .cover
                    .map(|(id, kind, is_complete)| LiteModule {
                        id,
                        kind,
                        is_complete,
                    }),
                modules: jig_data_row
                    .modules
                    .into_iter()
                    .map(|(id, kind, is_complete)| LiteModule {
                        id,
                        kind,
                        is_complete,
                    })
                    .collect(),
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

    Ok(res)
}

pub async fn browse_user_resources(
    pool: &PgPool,
    user_id: Uuid,
    page: u32,
    page_limit: u64,
) -> sqlx::Result<Vec<AdditionalResource>> {
    let mut txn = pool.begin().await?;

    let resources = sqlx::query!(
        r#"
        with cte as (
            select jdar.id              as "id",
                    jdar.display_name,
                   resource_type_id,
                   resource_content,
                   author_id,
                   updated_at,
                   created_at
            from jig_data_additional_resource "jdar"
            inner join jig on jig.live_id = jdar.jig_data_id
            inner join jig_data on jig.live_id = jig_data.id
            where author_id = $1
           ),
           cte1 as (
              select cdr.id              as "id",
                cdr.display_name,
                resource_type_id,
                resource_content,
                author_id,
                updated_at,
                created_at
          from course_data_resource "cdr"
          inner join course on course.live_id = cdr.course_data_id
          inner join course_data on course.live_id = course_data.id
          where author_id = $1
          )
          select id                     as "id!: AddId", 
                 display_name           as "display_name!",
                 resource_type_id       as "resource_type_id!: TypeId",
                 resource_content        as "resource_content!"
         from
          (select * from cte
          union all
          select * from cte1) cte2
          order by coalesce(updated_at, created_at)
          offset $2
          limit $3
            "#,
        user_id,
        page as i32,
        page_limit as i32
    )
    .fetch_all(&mut txn)
    .await?;

    let res: Vec<_> = resources
        .into_iter()
        .map(|row| AdditionalResource {
            id: row.id,
            display_name: row.display_name,
            resource_type_id: row.resource_type_id,
            resource_content: serde_json::from_value::<ResourceContent>(row.resource_content)
                .unwrap(),
        })
        .collect();

    txn.rollback().await?;

    Ok(res)
}

pub async fn browse_user_courses(
    pool: &PgPool,
    user_id: Uuid,
    page: u32,
    page_limit: u64,
) -> sqlx::Result<Vec<CourseResponse>> {
    let mut txn = pool.begin().await?;

    let course_data = sqlx::query!(
        //language=SQL
        r#"
    with cte as (
        select array(select cd.id as "id!"
        from course_data "cd"
              left join course on live_id = cd.id and course.published_at is not null
              left join course_data_resource "resource" on cd.id = resource.course_data_id
        where author_id = $1
        order by coalesce(updated_at, created_at) desc) as id
    ),
    cte1 as (
        select * from unnest((select distinct id from cte)) with ordinality t(id
       , ord) order by ord
    )
    select course.id                                                                 as "course_id: CourseId",
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
        (
            select row(course_data_module.id, kind, is_complete) 
            from course_data_module                
            where course_data_id = course_data.id and "index" = 0
            order by "index"
        )                                                   as "cover?: (ModuleId, ModuleKind, bool)",
        array(select row (category_id)
                from course_data_category
                where course_data_id = course_data.id)     as "categories!: Vec<(CategoryId,)>",
        array(select row (affiliation_id)
                from course_data_affiliation
                where course_data_id = course_data.id)     as "affiliations!: Vec<(AffiliationId,)>",
        array(select row (age_range_id)
                from course_data_age_range
                where course_data_id = course_data.id)     as "age_ranges!: Vec<(AgeRangeId,)>",
        array(
                    select row (cdr.id, cdr.display_name, resource_type_id, resource_content)
                    from course_data_resource "cdr"
                    where cdr.course_data_id = course_data.id
                )                                               as "additional_resource!: Vec<(AddId, String, TypeId, Value)>",
        array(
            select row(jig_id)
            from course_data_jig
            where course_data_jig.course_data_id = course_data.id
        )                                                     as "items!: Vec<JigId>"
    from cte1
    left join course_data on cte1.id = course_data.id
    left join course on course_data.id = course.live_id and course.published_at is not null
    where cte1.ord > (1 * $2 * $3)
    limit $3
    "#,
        user_id,
        page as i32,
        page_limit as i32,
    )
        .fetch_all(&mut txn)
        .await?;

    let res: Vec<_> = course_data
        .into_iter()
        .map(|course_data_row| CourseResponse {
            id: course_data_row.course_id,
            published_at: course_data_row.published_at,
            creator_id: course_data_row.creator_id,
            author_id: course_data_row.author_id,
            author_name: course_data_row.author_name,
            likes: course_data_row.likes,
            plays: course_data_row.plays,
            course_data: CourseData {
                draft_or_live: course_data_row.draft_or_live,
                display_name: course_data_row.display_name,
                language: course_data_row.language,
                cover: course_data_row
                    .cover
                    .map(|(id, kind, is_complete)| LiteModule {
                        id,
                        kind,
                        is_complete,
                    }),
                categories: course_data_row
                    .categories
                    .into_iter()
                    .map(|(it,)| it)
                    .collect(),
                last_edited: course_data_row.updated_at,
                description: course_data_row.description,
                age_ranges: course_data_row
                    .age_ranges
                    .into_iter()
                    .map(|(it,)| it)
                    .collect(),
                affiliations: course_data_row
                    .affiliations
                    .into_iter()
                    .map(|(it,)| it)
                    .collect(),
                additional_resources: course_data_row
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
                privacy_level: course_data_row.privacy_level,
                other_keywords: course_data_row.other_keywords,
                translated_keywords: course_data_row.translated_keywords,
                translated_description: course_data_row.translated_description.0,
                items: course_data_row.items,
            },
        })
        .collect();

    txn.rollback().await?;

    Ok(res)
}

pub async fn follow(
    pool: &PgPool,
    user_id: Uuid,
    follower_id: Uuid,
) -> anyhow::Result<(), error::NotFound> {
    let mut txn = pool.begin().await?;

    let _ = sqlx::query!(
        r#"
    select exists(select 1 from "user" where id = $1) as "exists!"
        "#,
        user_id
    )
    .fetch_optional(&mut txn)
    .await
    .map_err(|_| anyhow::anyhow!("User does not exist"))?;

    sqlx::query!(
        r#"
    insert into user_follow(user_id, follower_id, followed_at)
    values($1, $2, now())
            "#,
        user_id,
        follower_id
    )
    .execute(&mut txn)
    .await
    .map_err(|_| anyhow::anyhow!("User already follows target user"))?;

    txn.rollback().await?;

    Ok(())
}

pub async fn unfollow(
    pool: &PgPool,
    user_id: Uuid,
    follower_id: Uuid,
) -> anyhow::Result<(), error::NotFound> {
    let mut txn = pool.begin().await?;

    let _ = sqlx::query!(
        r#"
    select exists(select 1 from user_follow where user_id = $1 and follower_id = $2) as "exists!"
        "#,
        user_id,
        follower_id
    )
    .fetch_optional(&mut txn)
    .await
    .map_err(|_| anyhow::anyhow!("User does not follow target user"))?;

    sqlx::query!(
        r#"
    delete from user_follow where user_id = $1 and follower_id = $2
            "#,
        user_id,
        follower_id
    )
    .execute(&mut txn)
    .await
    .map_err(|_| anyhow::anyhow!("Could not unfollow user"))?;

    txn.rollback().await?;

    Ok(())
}

pub async fn browse_followers(
    pool: &PgPool,
    user_id: Uuid,
    page: u32,
    page_limit: u64,
) -> sqlx::Result<Vec<PublicUser>> {
    let mut txn = pool.begin().await?;

    let user_data = sqlx::query!(
        r#"
        with followers as (
            select follower_id 
            from user_follow 
            where user_id = $1
            order by coalesce(followed_at) desc
        )
        select  id         as "id!",
                username               as "username!",
                given_name             as "given_name!",
                family_name            as "family_name!",
                bio                    as "bio!",
                profile_image_id       as "profile_image?: ImageId",
                (select language from user_profile where user_profile.user_id = "user".id and language_public is true)      as "language?",
                (select organization from user_profile where user_profile.user_id = "user".id and organization_public is true)  as "organization?",
                (select array(select persona from user_profile where user_profile.user_id = "user".id and persona_public is true))      as "persona!: Vec<String>",
                (select location from user_profile where user_profile.user_id = "user".id and location_public is true)      as "location?",
                array(select badge.id
                    from badge_member bm
                    inner join badge on bm.id = badge.id
                    where bm.user_id = "user".id
                ) as "badges!: Vec<BadgeId>"
            from "user"
            inner join user_profile on "user".id = user_profile.user_id
            inner join followers on (followers.follower_id = "user".id)
            offset $2
            limit $3
            "#,
            user_id,
            page as i32,
            page_limit as i32,
        )
            .fetch_all(&mut txn)
            .await?;

    let res: Vec<_> = user_data
        .into_iter()
        .map(|row| PublicUser {
            id: row.id,
            username: row.username,
            given_name: row.given_name,
            family_name: row.family_name,
            bio: row.bio,
            profile_image: row.profile_image,
            language: row.language,
            organization: row.organization,
            persona: row.persona,
            location: row.location,
            badges: row.badges,
        })
        .collect();

    txn.rollback().await?;

    Ok(res)
}

pub async fn browse_following(
    pool: &PgPool,
    user_id: Uuid,
    page: u32,
    page_limit: u64,
) -> sqlx::Result<Vec<PublicUser>> {
    let mut txn = pool.begin().await?;

    let user_data = sqlx::query!(
        r#"
        with following as (
            select user_id 
            from user_follow 
            where follower_id = $1
            order by coalesce(followed_at) desc
        )
        select  id                     as "id!",
                username               as "username!",
                given_name             as "given_name!",
                family_name            as "family_name!",
                bio                    as "bio!",
                profile_image_id       as "profile_image?: ImageId",
                (select language from user_profile where user_profile.user_id = "user".id and language_public is true)      as "language?",
                (select organization from user_profile where user_profile.user_id = "user".id and organization_public is true)  as "organization?",
                (select array(select persona from user_profile where user_profile.user_id = "user".id and persona_public is true))      as "persona!",
                (select location from user_profile where user_profile.user_id = "user".id and location_public is true)      as "location?",
                array(select badge.id
                    from badge_member bm
                    inner join badge on bm.id = badge.id
                    where bm.user_id = "user".id
                ) as "badges!: Vec<BadgeId>"
            from "user"
            inner join user_profile on "user".id = user_profile.user_id
            inner join following on (following.user_id = "user".id)
            offset $2
            limit $3
            "#,
            user_id,
            page as i32,
            page_limit as i32,
        )
            .fetch_all(&mut txn)
            .await?;

    let res: Vec<_> = user_data
        .into_iter()
        .map(|row| PublicUser {
            id: row.id,
            username: row.username,
            given_name: row.given_name,
            family_name: row.family_name,
            bio: row.bio,
            profile_image: row.profile_image,
            language: row.language,
            organization: row.organization,
            persona: row.persona,
            location: row.location,
            badges: row.badges,
        })
        .collect();

    txn.rollback().await?;

    Ok(res)
}

pub async fn total_user_count(db: &PgPool) -> anyhow::Result<u64> {
    let user = sqlx::query!(
        r#"
        select count(*)             as "count!: i64"
            from "user"
        inner join user_profile on "user".id = user_profile.user_id
            "#
    )
    .fetch_one(db)
    .await?;

    Ok(user.count as u64)
}

pub async fn total_jig_count(db: &PgPool, user_id: Uuid) -> anyhow::Result<u64> {
    let total_jig = sqlx::query!(
        r#"
        select count(distinct jig.id) as "count!: i64"
            from jig
        left join jig_admin_data "admin" on admin.jig_id = jig.id
        left join jig_data on live_id = jig_data.id and published_at is not null and last_synced_at is not null
        left join jig_data_additional_resource "resource" on jig_data.id = resource.jig_data_id
        where author_id = $1
            "#,
            user_id
    )
    .fetch_one(db)
    .await?;

    Ok(total_jig.count as u64)
}

pub async fn total_resource_count(db: &PgPool, user_id: Uuid) -> sqlx::Result<u64> {
    let total_resource = sqlx::query!(
        r#"
        with cte as (
            select jdar.id              as "id"
            from jig_data_additional_resource "jdar"
            left join jig on jig.live_id = jdar.jig_data_id
            left join jig_data on jig.live_id = jig_data.id
            where author_id = $1
           ),
           cte1 as (
            select cdr.id              as "id"
          from course_data_resource "cdr"
          left join course on course.live_id = cdr.course_data_id
          left join course_data on course.live_id = course_data.id
          where author_id = $1
          )
          select count(id)                     as "count!: i32"
         from 
          (select * from cte
          union all
          select * from cte1) cte2
            "#,
        user_id
    )
    .fetch_one(db)
    .await?;

    Ok(total_resource.count as u64)
}

pub async fn total_course_count(db: &PgPool, user_id: Uuid) -> sqlx::Result<u64> {
    let total_jig = sqlx::query!(
        r#"
        select count(distinct course.id) as "count!: i64"
            from course
        left join course_data on live_id = course_data.id and published_at is not null
        left join course_data_resource "resource" on course_data.id = resource.course_data_id
        where author_id = $1
            "#,
        user_id
    )
    .fetch_one(db)
    .await?;

    Ok(total_jig.count as u64)
}

pub async fn total_follower_count(db: &PgPool, user_id: Uuid) -> sqlx::Result<u64> {
    let total_follower = sqlx::query!(
        r#"
        select count(follower_id)  as "count!: i64"
        from user_follow 
        where user_id = $1
            "#,
        user_id
    )
    .fetch_one(db)
    .await?;

    Ok(total_follower.count as u64)
}

pub async fn total_following_count(db: &PgPool, follower_id: Uuid) -> sqlx::Result<u64> {
    let total_following = sqlx::query!(
        r#"
        select count(user_id)  as "count!: i64"
        from user_follow 
        where follower_id = $1
            "#,
        follower_id
    )
    .fetch_one(db)
    .await?;

    Ok(total_following.count as u64)
}
