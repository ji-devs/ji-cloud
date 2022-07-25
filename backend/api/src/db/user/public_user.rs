use crate::{error, extractor::TokenUser};
// use serde_json::value::Value;
use shared::domain::{
    additional_resource::{AdditionalResource, AdditionalResourceId as AddId, ResourceContent},
    asset::UserOrMe,
    circle::CircleId,
    image::ImageId,
    meta::ResourceTypeId as TypeId,
    user::{public_user::PublicUser, UserId},
};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn get(db: &PgPool, user_id: UserId) -> anyhow::Result<Option<PublicUser>> {
    let profile = sqlx::query!(
        r#"
    select  user_id as "id!: UserId",
            username   as "username!",
            given_name  as "given_name!",
            family_name as "family_name!",
            profile_image_id       as "profile_image?: ImageId",
            (select language from user_profile where user_profile.user_id = "user".id and language_public is true)      as "language?",
            (select organization from user_profile where user_profile.user_id = "user".id and organization_public is true)  as "organization?",
            (select persona from user_profile where user_profile.user_id = "user".id and persona_public is true)      as "persona?: Vec<String>",
            (select location from user_profile where user_profile.user_id = "user".id and location_public is true)      as "location?",
            (select bio from user_profile where user_profile.user_id = "user".id and bio_public is true)      as "bio?",
            array(select circle.id
                from circle_member bm
                inner join circle on bm.id = circle.id
                where bm.user_id = "user".id
            ) as "circles!: Vec<CircleId>"
        from "user"
            inner join user_profile on "user".id = user_profile.user_id
        where id = $1
        "#,
        user_id.0
    )
    .fetch_optional(db)
    .await?;

    let res = profile.map(|row| PublicUser {
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
        circles: row.circles,
    });

    Ok(res)
}

pub async fn browse_users(
    pool: &PgPool,
    page: u32,
    page_limit: u64,
    circles: Vec<CircleId>,
) -> anyhow::Result<Vec<PublicUser>> {
    let mut txn = pool.begin().await?;

    let circle_ids = filters_for_ids_or(&circles[..]);

    let user_data = sqlx::query!(
        r#"
        with cte as (
            select (array_agg("user".id))[1]
            from "user"
            left join user_profile "up" on up.user_id = "user".id 
            left join circle_member "cm" on cm.user_id = "user".id
            left join circle on circle.id = cm.id 
            where cm.id = any($1) or $1 = array[]::uuid[]
            group by "user".created_at
            order by "user".created_at desc
        ),
        cte1 as (
            select * from unnest((select array_agg(cte.array_agg) from cte)) with ordinality t(id
           , ord) order by ord
        )
        select  user_id                as "id!: UserId",
                username               as "username!",
                given_name             as "given_name!",
                family_name            as "family_name!",
                profile_image_id       as "profile_image?: ImageId",
                (select language from user_profile where user_profile.user_id = "user".id and language_public is true)      as "language?",
                (select organization from user_profile where user_profile.user_id = "user".id and organization_public is true)  as "organization?",
                (select persona from user_profile where user_profile.user_id = "user".id and persona_public is true)      as "persona?: Vec<String>",
                (select location from user_profile where user_profile.user_id = "user".id and location_public is true)      as "location?",
                (select bio from user_profile where user_profile.user_id = "user".id and bio_public is true)      as "bio?",
                (select array(select circle.id
                    from circle_member bm
                    inner join circle on bm.id = circle.id
                    where bm.user_id = "user".id
                )) as "circles!: Vec<CircleId>"
        from cte1
        left join user_profile on cte1.id = user_profile.user_id
        left join "user" on cte1.id = "user".id
        where ord > (1 * $2 * $3)
        order by ord
        limit $3
            "#,
            &circle_ids[..],
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
            circles: row.circles,
        })
        .collect();

    txn.rollback().await?;

    Ok(res)
}

pub async fn browse_user_resources(
    db: &PgPool,
    user_id: UserId,
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
          select cte3.id                as "id!: AddId",
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
        user_id.0,
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
    user_id: UserId,
    follower_id: UserId,
) -> anyhow::Result<(), error::NotFound> {
    let mut txn = pool.begin().await?;

    let exists = sqlx::query!(
        r#"
    select exists(select 1 from user_follow where user_id = $1 and follower_id = $2) as "exists!"
        "#,
        user_id.0,
        follower_id.0
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
        user_id.0,
        follower_id.0
    )
    .execute(&mut txn)
    .await
    .map_err(|_| anyhow::anyhow!("User already follows target user"))?;

    txn.commit().await?;

    Ok(())
}

pub async fn unfollow(
    pool: &PgPool,
    user_id: UserId,
    follower_id: UserId,
) -> anyhow::Result<(), error::NotFound> {
    let mut txn = pool.begin().await?;

    let exists = sqlx::query!(
        r#"
    select exists(select 1 from user_follow where user_id = $1 and follower_id = $2) as "exists!"
        "#,
        user_id.0,
        follower_id.0
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
        user_id.0,
        follower_id.0
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
        select  user_id                as "id!: UserId",
                username               as "username!",
                given_name             as "given_name!",
                family_name            as "family_name!",
                profile_image_id       as "profile_image?: ImageId",
                (select language from user_profile where user_profile.user_id = "user".id and language_public is true)      as "language?",
                (select organization from user_profile where user_profile.user_id = "user".id and organization_public is true)  as "organization?",
                (select persona from user_profile where user_profile.user_id = "user".id and persona_public is true)      as "persona?: Vec<String>",
                (select location from user_profile where user_profile.user_id = "user".id and location_public is true)      as "location?",
                (select bio from user_profile where user_profile.user_id = "user".id and bio_public is true)      as "bio?",
                (select array(select circle.id
                    from circle_member bm
                    inner join circle on bm.id = circle.id
                    where bm.user_id = "user".id
                )) as "circles!: Vec<CircleId>"
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
            circles: row.circles,
        })
        .collect();

    txn.rollback().await?;

    Ok(v)
}

pub(crate) async fn auth_claims(
    db: &PgPool,
    claims: Option<TokenUser>,
    user_id: Option<UserOrMe>,
) -> Result<Option<UserId>, error::Auth> {
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
            user_id.map(|x| UserId(x))
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
            user.map(|x| UserId(x))
        } else {
            None
        };
        id
    };

    Ok(id)
}

pub async fn browse_followers(
    pool: &PgPool,
    user_id: UserId,
    page: u32,
    page_limit: u64,
) -> sqlx::Result<Vec<PublicUser>> {
    let mut txn = pool.begin().await?;

    let user_data = sqlx::query!(
        r#"
        with follower as (
            select (array_agg(follower_id))[1]
            from user_follow
            where user_id = $1
            group by followed_at
            order by followed_at desc
        ),
        cte as (
            select * from unnest((select array_agg(follower.array_agg) from follower)) with ordinality t(id, ord) order by ord
        )
        select  "user".id         as "id!: UserId",
                username               as "username!",
                given_name             as "given_name!",
                family_name            as "family_name!",
                profile_image_id       as "profile_image?: ImageId",
                (select language from user_profile where user_profile.user_id = "user".id and language_public is true)      as "language?",
                (select organization from user_profile where user_profile.user_id = "user".id and organization_public is true)  as "organization?",
                (select array(select persona from user_profile where user_profile.user_id = "user".id and persona_public is true))      as "persona?: Vec<String>",
                (select location from user_profile where user_profile.user_id = "user".id and location_public is true)      as "location?",
                (select bio from user_profile where user_profile.user_id = "user".id and bio_public is true)      as "bio?",
                (select array(select circle.id
                    from circle_member bm
                    left join circle on bm.id = circle.id
                    where bm.user_id = "user".id or circle.creator_id = "user".id
                )) as "circles!: Vec<CircleId>"
        from cte
        left join user_profile on cte.id = user_profile.user_id
        left join "user" on (cte.id = "user".id)
        where ord > (1 * $2 * $3)
        limit $3;

            "#,
            user_id.0,
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
            circles: row.circles,
        })
        .collect();

    txn.rollback().await?;

    Ok(res)
}

pub async fn browse_following(
    pool: &PgPool,
    user_id: UserId,
    page: u32,
    page_limit: u64,
) -> sqlx::Result<Vec<PublicUser>> {
    let mut txn = pool.begin().await?;

    let user_data = sqlx::query!(
        r#"
        with following as (
            select (array_agg(user_id))[1]
            from user_follow
            where follower_id = $1
            group by followed_at
            order by followed_at desc
        ),
        cte as (
            select * from unnest((select array_agg(following.array_agg) from following)) with ordinality t(id, ord) order by ord
        )
        select  "user".id                     as "id!: UserId",
                username               as "username!",
                given_name             as "given_name!",
                family_name            as "family_name!",
                profile_image_id       as "profile_image?: ImageId",
                (select language from user_profile where user_profile.user_id = "user".id and language_public is true)      as "language?",
                (select organization from user_profile where user_profile.user_id = "user".id and organization_public is true)  as "organization?",
                (select persona from user_profile where user_profile.user_id = "user".id and persona_public is true)      as "persona?: Vec<String>",
                (select location from user_profile where user_profile.user_id = "user".id and location_public is true)      as "location?",
                (select bio from user_profile where user_profile.user_id = "user".id and bio_public is true)      as "bio?",
                array(select circle.id
                    from circle_member bm
                    left join circle on bm.id = circle.id
                    where bm.user_id = "user".id or circle.creator_id = "user".id
                ) as "circles!: Vec<CircleId>"
            from cte
            left join user_profile on cte.id = user_profile.user_id
            left join "user" on (cte.id = "user".id)
            where ord > (1 * $2 * $3)
            limit $3;
            "#,
            user_id.0,
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
            circles: row.circles,
        })
        .collect();

    txn.rollback().await?;

    Ok(res)
}

pub async fn total_user_count(db: &PgPool, circles: Vec<CircleId>) -> anyhow::Result<u64> {
    let circle_ids = filters_for_ids_or(&circles[..]);

    let user = sqlx::query!(
        r#"
        select count(*)             as "count!: i64"
            from "user"
        left join user_profile "up" on up.user_id = "user".id 
        left join circle_member "cm" on cm.user_id = "user".id
        left join circle on circle.id = cm.id 
        where cm.id = any($1) or $1 = array[]::uuid[]
        "#,
        &circle_ids[..]
    )
    .fetch_one(db)
    .await?;

    Ok(user.count as u64)
}

pub async fn total_resource_count(db: &PgPool, user_id: UserId) -> sqlx::Result<u64> {
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
        user_id.0
    )
    .fetch_one(db)
    .await?;

    Ok(total_resource.count as u64)
}

pub async fn total_follower_count(db: &PgPool, user_id: UserId) -> sqlx::Result<u64> {
    let total_follower = sqlx::query!(
        r#"
        select count(follower_id)  as "count!: i64"
        from user_follow
        where user_id = $1
            "#,
        user_id.0
    )
    .fetch_one(db)
    .await?;

    Ok(total_follower.count as u64)
}

pub async fn total_following_count(db: &PgPool, follower_id: UserId) -> sqlx::Result<u64> {
    let total_following = sqlx::query!(
        r#"
        select count(user_id)  as "count!: i64"
        from user_follow
        where follower_id = $1
            "#,
        follower_id.0
    )
    .fetch_one(db)
    .await?;

    Ok(total_following.count as u64)
}

fn filters_for_ids_or<T: Into<Uuid> + Copy>(ids: &[T]) -> Vec<Uuid> {
    let mut vect: Vec<Uuid> = vec![];
    for id in ids.iter().copied() {
        let id: Uuid = id.into();
        vect.push(id);
    }

    vect
}
