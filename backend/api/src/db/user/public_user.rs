use crate::{error, extractor::TokenUser};
// use serde_json::value::Value;
use shared::domain::{
    additional_resource::{AdditionalResource, AdditionalResourceId as AddId, ResourceContent},
    asset::UserOrMe,
    badge::BadgeId,
    image::ImageId,
    meta::ResourceTypeId as TypeId,
    user::public_user::PublicUser,
};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get(db: &PgPool, user_id: Uuid) -> sqlx::Result<PublicUser> {
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
    .fetch_one(db)
    .await?;

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
        with cte as (
            select array(select id  as "id!"
            from "user"
            inner join user_profile "up" on "user".id = up.user_id
            order by family_name asc) as id
        ),
        cte1 as (
            select * from unnest((select distinct id from cte)) with ordinality t(id
           , ord) order by ord
        )
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
                (select array(select badge.id 
                    from badge_member bm 
                    inner join badge on bm.id = badge.id 
                    where bm.user_id = "user".id
                )) as "badges!: Vec<(BadgeId,)>"
            from "user"
            inner join user_profile on "user".id = user_profile.user_id
            inner join cte1 on cte1.id = "user".id
            where ord > (1 * $1 * $2)
            order by ord 
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
            badges: row.badges.into_iter().map(|(x,)| x).collect(),
        })
        .collect();

    txn.rollback().await?;

    Ok(res)
}

pub async fn browse_user_resources(
    db: &PgPool,
    user_id: Uuid,
    page: u32,
    page_limit: u64,
) -> sqlx::Result<Vec<AdditionalResource>> {
    let resources = sqlx::query!(
        r#"
        with cte as (
            select distinct jdar.id              as id,
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
              select distinct cdr.id              as id,
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
          ),
          cte2 as (
            select * from unnest(array(select id from (select id from cte union all select id from cte1) resource)) with ordinality t(id
           , ord) order by ord
         )
          select cte3.id                     as "id!: AddId",
                 display_name           as "display_name!",
                 resource_type_id       as "resource_type_id!: TypeId",
                 resource_content        as "resource_content!"
         from
          (select * from cte
          union all
          select * from cte1) cte3
        inner join cte2 on cte2.id = cte3.id
        where ord > (1 * $2 * $3)
        order by ord asc
        limit $3
            "#,
        user_id,
        page as i32,
        page_limit as i32
    )
    .fetch_all(db)
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

    Ok(res)
}

pub async fn follow(
    pool: &PgPool,
    user_id: Uuid,
    follower_id: Uuid,
) -> anyhow::Result<(), error::NotFound> {
    let mut txn = pool.begin().await?;

    let exists = sqlx::query!(
        r#"
    select exists(select 1 from user_follow where user_id = $1 and follower_id = $2) as "exists!"
        "#,
        user_id,
        follower_id
    )
    .fetch_one(&mut txn)
    .await?
    .exists;

    if exists {
        return Err(error::NotFound::InternalServerError(anyhow::anyhow!(
            "User already follows target user"
        )));
    };

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

    txn.commit().await?;

    Ok(())
}

pub async fn unfollow(
    pool: &PgPool,
    user_id: Uuid,
    follower_id: Uuid,
) -> anyhow::Result<(), error::NotFound> {
    let mut txn = pool.begin().await?;

    let exists = sqlx::query!(
        r#"
    select exists(select 1 from user_follow where user_id = $1 and follower_id = $2) as "exists!"
        "#,
        user_id,
        follower_id
    )
    .fetch_one(&mut txn)
    .await?
    .exists;

    if !exists {
        return Err(error::NotFound::InternalServerError(anyhow::anyhow!(
            "User does not follow target user"
        )));
    };

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

    txn.commit().await?;

    Ok(())
}

pub async fn get_by_ids(db: &PgPool, ids: &[Uuid]) -> sqlx::Result<Vec<PublicUser>> {
    let mut txn = db.begin().await?;

    let res: Vec<_> = sqlx::query!(
        //language=SQL
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
                (select array(select badge.id 
                    from badge_member bm 
                    inner join badge on bm.id = badge.id 
                    where bm.user_id = "user".id
                )) as "badges!: Vec<(BadgeId,)>"
                from "user"
                inner join user_profile on "user".id = user_profile.user_id
                inner join unnest($1::uuid[])
                with ordinality t(id, ord) using (id)
"#,
        ids
    )
    .fetch_all(&mut txn)
    .await?;

    let v = res
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
            badges: row.badges.into_iter().map(|(x,)| x).collect(),
        })
        .collect();

    txn.rollback().await?;

    Ok(v)
}

pub(crate) async fn auth_claims(
    db: &PgPool,
    claims: Option<TokenUser>,
    user_id: Option<UserOrMe>,
) -> Result<Option<Uuid>, error::Auth> {
    //Check if user is logged in. If not, users cannot use UserOrMe::Me
    let id = if let Some(token) = claims {
        let id = if let Some(user) = user_id {
            let user_id = match user {
                UserOrMe::Me => Some(token.0.user_id),
                UserOrMe::User(id) => {
                    if !sqlx::query!(
                        //language=SQL
                        r#"
            select exists(select 1 from user_profile where user_id = $1 for update) as "exists!"
                "#,
                        id
                    )
                    .fetch_one(db)
                    .await?
                    .exists
                    {
                        return Err(error::Auth::ResourceNotFound(
                            "user Id does not exist".to_string(),
                        ));
                    }

                    Some(id)
                }
            };
            user_id
        } else {
            None
        };
        id
    } else {
        let id = if let Some(user) = user_id {
            let user = match user {
                UserOrMe::Me => return Err(error::Auth::Forbidden),
                UserOrMe::User(id) => {
                    if !sqlx::query!(
                        //language=SQL
                        r#"
                select exists(select 1 from user_profile where user_id = $1 for update) as "exists!"
                    "#,
                        id
                    )
                    .fetch_one(db)
                    .await?
                    .exists
                    {
                        return Err(error::Auth::ResourceNotFound(
                            "user Id does not exist".to_string(),
                        ));
                    }

                    Some(id)
                }
            };
            user
        } else {
            None
        };
        id
    };

    Ok(id)
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
                (select array(select badge.id
                    from badge_member bm
                    left join badge on bm.id = badge.id
                    where bm.user_id = "user".id or badge.creator_id = "user".id 
                )) as "badges!: Vec<BadgeId>"
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
                    left join badge on bm.id = badge.id
                    where bm.user_id = "user".id or badge.creator_id = "user".id 
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

pub async fn total_resource_count(db: &PgPool, user_id: Uuid) -> sqlx::Result<u64> {
    let total_resource = sqlx::query!(
        r#"
        with cte as (
            select distinct jdar.id             as "id",
                   updated_at,
                   created_at
            from jig_data_additional_resource "jdar"
            inner join jig on jig.live_id = jdar.jig_data_id
            inner join jig_data on jig.live_id = jig_data.id
            where author_id = $1
           ),
           cte1 as (
              select  distinct cdr.id          as "id",
                      updated_at,
                      created_at
          from course_data_resource "cdr"
          inner join course on course.live_id = cdr.course_data_id
          inner join course_data on course.live_id = course_data.id
          where author_id = $1
          )
          select count(id)                     as "count!: i64"
         from
          (select * from cte
          union all
          select * from cte1) resources"#,
        user_id
    )
    .fetch_one(db)
    .await?;

    Ok(total_resource.count as u64)
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
