use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use shared::domain::{
    category::CategoryId,
    jig::{
        additional_resource::AdditionalResourceId, module::ModuleId, Jig, JigId, LiteModule,
        ModuleKind,
    },
    meta::{AffiliationId, AgeRangeId, GoalId},
    user::UserScope,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error;

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
) -> sqlx::Result<JigId> {
    let mut transaction = pool.begin().await?;

    let jig = sqlx::query!(
        r#"
insert into jig
    (display_name, creator_id, author_id, publish_at, language, description)
values ($1, $2, $2, $3, $4, $5)
returning id
"#,
        display_name,
        creator_id,
        publish_at,
        language,
        description,
    )
    .fetch_one(&mut transaction)
    .await?;

    super::recycle_metadata(&mut transaction, "jig", jig.id, goals).await?;
    super::recycle_metadata(&mut transaction, "jig", jig.id, categories).await?;
    super::recycle_metadata(&mut transaction, "jig", jig.id, age_ranges).await?;
    super::recycle_metadata(&mut transaction, "jig", jig.id, affiliations).await?;

    let default_modules = [
        (Some(ModuleKind::Cover), Some(serde_json::json!({}))),
        (None, None),
    ];

    // todo: batch
    for (idx, (kind, contents)) in default_modules.iter().enumerate() {
        sqlx::query!(
            r#"
insert into jig_module (jig_id, "index", kind, contents)
values ($1, $2, $3, $4)"#,
            jig.id,
            idx as i16,
            kind.map(|it| it as i16),
            contents.as_ref()
        )
        .execute(&mut transaction)
        .await?;
    }

    transaction.commit().await?;

    Ok(JigId(jig.id))
}

pub async fn get_by_ids(db: &PgPool, ids: &[Uuid]) -> sqlx::Result<Vec<Jig>> {
    let v = sqlx::query!(
r#"
select  
    id as "id: JigId",
    display_name,
    creator_id,
    author_id,
    publish_at,
    updated_at,
    language,
    description,
    is_public,
    array(
        select row (id, kind)
        from jig_module
        where jig_id = jig.id
        order by "index"
    ) as "modules!: Vec<(ModuleId, Option<ModuleKind>)>",
    array(select row(goal_id) from jig_goal where jig_id = jig.id) as "goals!: Vec<(GoalId,)>",
    array(select row(category_id) from jig_category where jig_id = jig.id) as "categories!: Vec<(CategoryId,)>",
    array(select row(affiliation_id) from jig_affiliation where jig_id = jig.id) as "affiliations!: Vec<(AffiliationId,)>",
    array(select row(age_range_id) from jig_age_range where jig_id = jig.id) as "age_ranges!: Vec<(AgeRangeId,)>",
    array(select row(id) from jig_additional_resource where jig_id = jig.id) as "additional_resources!: Vec<(AdditionalResourceId,)>"
from jig
inner join unnest($1::uuid[]) with ordinality t(id, ord) USING (id)
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
            language: row.language,
            categories: row.categories.into_iter().map(|(it,)| it).collect(),
            publish_at: row.publish_at,
            description: row.description,
            last_edited: row.updated_at,
            is_public: row.is_public,
            age_ranges: row.age_ranges.into_iter().map(|(it,)| it).collect(),
            affiliations: row.affiliations.into_iter().map(|(it,)| it).collect(),
            additional_resources: row
                .additional_resources
                .into_iter()
                .map(|(it,)| it)
                .collect(),
        })
        .collect();

    Ok(v)
}

pub async fn get(pool: &PgPool, id: JigId) -> anyhow::Result<Option<Jig>> {
    let jig = sqlx::query!(
        r#"
select  
    id as "id: JigId",
    display_name,
    creator_id,
    author_id,
    publish_at,
    updated_at,
    language,
    description,
    is_public,
    array(
        select row (id, kind)
        from jig_module
        where jig_id = $1
        order by "index"
    ) as "modules!: Vec<(ModuleId, Option<ModuleKind>)>",
    array(select row(goal_id) from jig_goal where jig_id = $1) as "goals!: Vec<(GoalId,)>",
    array(select row(category_id) from jig_category where jig_id = $1) as "categories!: Vec<(CategoryId,)>",
    array(select row(affiliation_id) from jig_affiliation where jig_id = jig.id) as "affiliations!: Vec<(AffiliationId,)>",
    array(select row(age_range_id) from jig_age_range where jig_id = jig.id) as "age_ranges!: Vec<(AgeRangeId,)>",
    array(select row(id) from jig_additional_resource where jig_id = $1) as "additional_resources!: Vec<(AdditionalResourceId,)>"
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
        goals: row.goals.into_iter().map(|(it,)| it).collect(),
        categories: row.categories.into_iter().map(|(it,)| it).collect(),
        creator_id: row.creator_id,
        author_id: row.author_id,
        publish_at: row.publish_at,
        description: row.description,
        last_edited: row.updated_at,
        is_public: row.is_public,
        age_ranges: row.age_ranges.into_iter().map(|(it,)| it).collect(),
        affiliations: row.affiliations.into_iter().map(|(it,)| it).collect(),
        additional_resources: row.additional_resources.into_iter().map(|(it,) | it).collect(),
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
    is_public: Option<bool>,
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

    if let Some(publish_at) = publish_at {
        sqlx::query!(
            r#"
update jig
set publish_at = $2, updated_at = now()
where id = $1 and $2 is distinct from publish_at"#,
            id.0,
            publish_at
        )
        .execute(&mut transaction)
        .await?;
    }

    sqlx::query!(
        r#"
update jig
set display_name    = coalesce($2, display_name),
    author_id       = coalesce($3, author_id),
    language        = coalesce($4, language),
    description     = coalesce($5, description),
    is_public       = coalesce($6, is_public),
    updated_at      = now()
where id = $1
  and (($2::text is not null and $2 is distinct from display_name) or
       ($3::uuid is not null and $3 is distinct from author_id) or
       ($4::text is not null and $4 is distinct from language) or
       ($5::text is not null and $5 is distinct from description) or
       ($6::bool is not null and $6 is distinct from is_public))"#,
        id.0,
        display_name,
        author_id,
        language,
        description,
        is_public,
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
    sqlx::query!("delete from jig where id = $1", id.0)
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
    sqlx::query!(
        r#"
select  
    id as "id: JigId",
    display_name,
    creator_id,
    author_id,
    publish_at,
    updated_at,
    language,
    description,
    is_public,
    array(
        select row (id, kind)
        from jig_module
        where jig_id = jig.id
        order by "index"
    ) as "modules!: Vec<(ModuleId, Option<ModuleKind>)>",
    array(select row(goal_id) from jig_goal where jig_id = jig.id) as "goals!: Vec<(GoalId,)>",
    array(select row(category_id) from jig_category where jig_id = jig.id) as "categories!: Vec<(CategoryId,)>",
    array(select row(affiliation_id) from jig_affiliation where jig_id = jig.id) as "affiliations!: Vec<(AffiliationId,)>",
    array(select row(age_range_id) from jig_age_range where jig_id = jig.id) as "age_ranges!: Vec<(AgeRangeId,)>",
    array(select row(id) from jig_additional_resource where jig_id = jig.id) as "additional_resources!: Vec<(AdditionalResourceId,)>"
from jig
where 
    publish_at < now() is not distinct from $1 or $1 is null
    and author_id is not distinct from $3 or $3 is null
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
        goals: row.goals.into_iter().map(|(it,)| it).collect(),
        categories: row.categories.into_iter().map(|(it,)| it).collect(),
        creator_id: row.creator_id,
        author_id: row.author_id,
        publish_at: row.publish_at,
        description: row.description,
        last_edited: row.updated_at,
        is_public: row.is_public,
        age_ranges: row.age_ranges.into_iter().map(|(it,)| it).collect(),
        affiliations: row.affiliations.into_iter().map(|(it,)| it).collect(),
        additional_resources: row.additional_resources.into_iter().map(|(it,) | it).collect(),
    })
    .try_collect()
    .await
}

pub async fn filtered_count(
    db: &PgPool,
    is_published: Option<bool>,
    author_id: Option<Uuid>,
) -> sqlx::Result<u64> {
    sqlx::query!(
        r#"
select count(*) as "count!: i64"
from jig
where
    publish_at < now() is not distinct from $1 or $1 is null
    and author_id is not distinct from $2 or $2 is null
"#,
        is_published,
        author_id,
    )
    .fetch_one(db)
    .await
    .map(|it| it.count as u64)
}

pub async fn clone(db: &PgPool, parent: JigId, user_id: Uuid) -> sqlx::Result<Option<JigId>> {
    let mut txn = db.begin().await?;

    let new_id = sqlx::query!(
        r#"
insert into jig (display_name, parents, creator_id, author_id, language, description)
select display_name, array_append(parents, id), $2 as creator_id, $2 as author_id, language, description
from jig
where id = $1
returning id
        "#,
        parent.0,
        user_id
    )
    .fetch_optional(&mut txn)
    .await?;

    let new_id = match new_id {
        Some(it) => it.id,
        None => return Ok(None),
    };

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

    txn.commit().await?;

    Ok(Some(JigId(new_id)))
}

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
