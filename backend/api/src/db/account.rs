use crate::db;
use shared::domain::admin::SearchSchoolNamesParams;
use shared::domain::billing::{
    Account, AccountId, AccountType, AccountUser, CustomerId, PaymentMethod, School, SchoolId,
    SchoolName, SchoolNameId, SchoolNameValue, SubscriptionStatus, SubscriptionTier,
    UpdateSchoolAccountRequest, UserAccountSummary,
};
use shared::domain::image::ImageId;
use shared::domain::user::UserId;
use shared::domain::ItemCount;
use sqlx::{Executor, PgPool, Postgres};
use tracing::{instrument, Instrument};

#[instrument(skip(pool))]
pub async fn get_verified_school_names(pool: &PgPool) -> sqlx::Result<Vec<SchoolName>> {
    sqlx::query_as!(
        SchoolName,
        //language=SQL
        r#"
select
    school_name_id as "id!: SchoolNameId",
    name::text as "name!",
    verified
from school_name
where verified = true
"#,
    )
    .fetch_all(pool)
    .await
}

#[instrument(skip(pool))]
pub async fn check_school_name_exists(pool: &PgPool, name: &str) -> sqlx::Result<bool> {
    sqlx::query_scalar!(
        //language=SQL
        r#"select exists(select 1 from school_name where citext_eq(name, $1::text::citext)) as "exists!""#,
        name.trim(),
    )
    .fetch_one(pool)
    .await
}

#[instrument(skip(pool))]
pub async fn check_renamed_school_name_exists(
    pool: &PgPool,
    name: &str,
    current_school_id: &SchoolId,
) -> sqlx::Result<bool> {
    sqlx::query_scalar!(
        //language=SQL
        r#"
select exists(
    select 1
    from school_name
    left join school using (school_name_id)
    where
        citext_eq(name, $1::text::citext)
        and (school_id is null or school_id != $2)
) as "exists!"
"#,
        name.trim(),
        current_school_id as &SchoolId,
    )
    .fetch_one(pool)
    .await
}

#[instrument(skip(pool))]
pub async fn check_user_has_account(pool: &PgPool, user_id: UserId) -> sqlx::Result<bool> {
    let exists = sqlx::query_scalar!(
        // language=SQL
        r#"
select exists(
    select 1 from user_account where user_id = $1
) as "exists!""#,
        user_id as UserId,
    )
    .fetch_one(pool)
    .await?;

    Ok(exists)
}

#[instrument(skip(pool))]
pub async fn add_school_name(
    pool: &PgPool,
    new_name: SchoolNameValue,
    verified: bool,
) -> sqlx::Result<SchoolNameId> {
    let new_name = String::from(new_name);

    sqlx::query_scalar!(
        // language=SQL
        r#"
insert into school_name (name, verified)
values ($1::text::citext, $2)
returning school_name_id as "school_name_id!: SchoolNameId"
"#,
        new_name.trim(),
        verified,
    )
    .fetch_one(pool)
    .await
}

#[instrument(skip(pool))]
pub async fn update_school_name(
    pool: &PgPool,
    school_name_id: &SchoolNameId,
    new_name: SchoolNameValue,
    verified: bool,
) -> sqlx::Result<()> {
    let new_name = String::from(new_name);

    sqlx::query!(
        // language=SQL
        r#"
update school_name
set
    name = $2::text::citext,
    verified = $3
where school_name_id = $1
"#,
        school_name_id as &SchoolNameId,
        new_name.trim(),
        verified,
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[instrument(skip(pool))]
pub async fn check_school_exists(
    pool: &PgPool,
    school_name_id: &SchoolNameId,
) -> sqlx::Result<bool> {
    sqlx::query_scalar!(
        // language=SQL
        r#"select exists(select 1 from school where school_name_id = $1) as "exists!""#,
        school_name_id as &SchoolNameId,
    )
    .fetch_one(pool)
    .await
}

#[instrument(skip(pool))]
pub async fn create_default_individual_account(
    pool: &PgPool,
    user_id: &UserId,
    subscription_tier: &SubscriptionTier,
) -> sqlx::Result<AccountId> {
    let mut txn = pool.begin().await?;
    // Create an account record
    let account_id = sqlx::query_scalar!(
        // language=SQL
        r#"insert into account (account_type) values ($1) returning account_id as "account_id!: AccountId""#,
        AccountType::Individual as AccountType,
    )
        .fetch_one(&mut txn)
        .await?;

    // Associate the user with the account and mark them as an administrator.
    associate_user_with_account(
        &mut txn,
        &user_id,
        &account_id,
        &subscription_tier,
        true,
        true,
    )
    .await?;

    txn.commit().await?;

    Ok(account_id)
}

#[instrument(skip(pool))]
pub async fn create_default_school_account(
    pool: &PgPool,
    user_id: UserId,
    school_name_id: SchoolNameId,
    location: serde_json::Value,
) -> sqlx::Result<SchoolId> {
    let mut txn = pool.begin().await?;

    // Create an account record
    let account_id = sqlx::query_scalar!(
        // language=SQL
        r#"insert into account (account_type) values ($1) returning account_id as "account_id!: AccountId""#,
        AccountType::School as AccountType,
    )
        .fetch_one(&mut txn)
        .await?;

    // Associate the user with the account and mark them as an administrator.
    associate_user_with_account(
        &mut txn,
        &user_id,
        &account_id,
        &SubscriptionTier::Pro,
        true,
        true,
    )
    .await?;

    // Create the school record
    let school_id = sqlx::query_scalar!(
        // language=SQL
        r#"
insert into school
(school_name_id, email, location, account_id)
values
($1, (select email from user_email where user_id=$2), $3, $4)
returning school_id as "school_id!: SchoolId"
"#,
        school_name_id as SchoolNameId,
        user_id as UserId,
        location,
        account_id as AccountId,
    )
    .fetch_one(&mut txn)
    .await?;

    txn.commit().await?;

    Ok(school_id)
}

#[instrument(skip(pool))]
pub async fn update_school_account(
    pool: &PgPool,
    school_id: &SchoolId,
    update: UpdateSchoolAccountRequest,
) -> sqlx::Result<()> {
    // Create the school record
    sqlx::query_scalar!(
        // language=SQL
        r#"
update school
    set
        location = coalesce($2, location),
        email = coalesce($3::text, email),
        description = coalesce($4, description),
        profile_image_id = coalesce($5, profile_image_id),
        website = coalesce($6, website),
        organization_type = coalesce($7, organization_type)
where school_id = $1
"#,
        school_id as &SchoolId,
        update.location,
        update.email,
        update.description,
        update.profile_image as Option<ImageId>,
        update.website,
        update.organization_type,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn associate_user_with_account<'c, E: Executor<'c, Database = Postgres>>(
    executor: E,
    user_id: &UserId,
    account_id: &AccountId,
    subscription_tier: &SubscriptionTier,
    admin_user: bool,
    verified: bool,
) -> sqlx::Result<()> {
    // Associate the user with the account and mark them as an administrator.
    sqlx::query!(
        // language=SQL
        r#"
insert into user_account
(user_id, account_id, subscription_tier, admin, verified)
values
($1, $2, $3, $4, $5)"#,
        user_id as &UserId,
        account_id as &AccountId,
        subscription_tier as &SubscriptionTier,
        admin_user,
        verified,
    )
    .execute(executor)
    .await?;

    Ok(())
}

#[instrument(skip(pool))]
pub async fn get_user_account_summary(
    pool: &PgPool,
    user_id: &UserId,
) -> sqlx::Result<Option<UserAccountSummary>> {
    sqlx::query_as!(
        UserAccountSummary,
        // language=SQL
        r#"
select
    account.account_type as "account_type!: AccountType",
    user_account.subscription_tier as "subscription_tier?: SubscriptionTier",
    subscription.status as "subscription_status?: SubscriptionStatus",
    user_account.admin as "is_admin!",
    user_account.verified as "verified!",
    case
        when subscription.amount_due > 0 then true
        else false
    end as "overdue!"
from user_account
inner join account using (account_id)
left join (
    select subscription.account_id, status, amount_due
    from subscription
    join (
        select
            account_id, subscription_id, max(created_at)
        from subscription
        group by account_id, subscription_id
        limit 1
    ) as recent_subscription using (subscription_id)
) as subscription using (account_id)
where user_account.user_id = $1
"#,
        user_id as &UserId,
    )
    .fetch_optional(pool)
    .await
}

#[instrument(skip(pool))]
pub async fn get_account_by_user_id(
    pool: &PgPool,
    user_id: &UserId,
) -> anyhow::Result<Option<Account>> {
    let account = sqlx::query!(
        // language=SQL
        r#"
select
    account_id as "account_id!: AccountId",
    account_type as "account_type!: AccountType",
    stripe_customer_id as "stripe_customer_id?: CustomerId",
    payment_method as "payment_method?: serde_json::Value",
    created_at,
    updated_at
from account
join user_account using (account_id)
where user_account.user_id = $1
"#,
        user_id as &UserId,
    )
    .fetch_optional(pool)
    .await?;

    match account {
        Some(account) => Ok(Some(Account {
            account_id: account.account_id,
            account_type: account.account_type,
            stripe_customer_id: account.stripe_customer_id,
            payment_method: match account.payment_method {
                Some(payment_method) => serde_json::from_value(payment_method)?,
                None => None,
            },
            subscription: db::billing::get_latest_subscription_by_account_id(
                pool,
                account.account_id,
            )
            .await?,
            created_at: account.created_at,
            updated_at: account.updated_at,
        })),
        None => Ok(None),
    }
}

#[instrument(skip(db))]
pub async fn save_customer_id(
    db: &PgPool,
    account_id: &AccountId,
    customer_id: &CustomerId,
) -> anyhow::Result<()> {
    sqlx::query!(
        //language=SQL
        r#"
update account
set stripe_customer_id = $2,
updated_at = now()
where account_id = $1"#,
        account_id.0,
        customer_id.as_str(),
    )
    .execute(db)
    .instrument(tracing::info_span!("set customer id"))
    .await?;

    Ok(())
}

#[instrument(skip(db))]
pub async fn save_payment_method(
    db: &PgPool,
    account_id: &AccountId,
    payment_method: Option<PaymentMethod>,
) -> anyhow::Result<()> {
    sqlx::query!(
        //language=SQL
        r#"
update account
set payment_method = $2,
updated_at = now()
where account_id = $1"#,
        account_id.0,
        serde_json::to_value(payment_method)?,
    )
    .execute(db)
    .await?;

    Ok(())
}

#[instrument(skip(db))]
pub async fn get_account_id_by_customer_id(
    db: &PgPool,
    customer_id: &CustomerId,
) -> anyhow::Result<Option<AccountId>> {
    Ok(sqlx::query_scalar!(
        //language=SQL
        r#"select account_id as "account_id: AccountId" from account where stripe_customer_id = $1"#,
        customer_id.as_str(),
    )
        .fetch_optional(db)
        .await?)
}

#[instrument(skip(db))]
pub async fn get_account_id_by_school_id(
    db: &PgPool,
    school_id: &SchoolId,
) -> anyhow::Result<Option<AccountId>> {
    Ok(sqlx::query_scalar!(
        //language=SQL
        r#"select account_id as "account_id: AccountId" from school where school_id = $1"#,
        school_id as &SchoolId,
    )
    .fetch_optional(db)
    .await?)
}

#[instrument(skip(pool))]
pub async fn get_school_account_by_account_id(
    pool: &PgPool,
    account_id: &AccountId,
) -> sqlx::Result<Option<School>> {
    let record = sqlx::query!(
        // language=SQL
        r#"
select
    school_id as "id!: SchoolId",
    school_name_id as "name!: SchoolNameId",
    location as "location?: serde_json::Value",
    email::text as "email!",
    description,
    profile_image_id as "profile_image?: ImageId",
    website,
    organization_type,
    account_id as "account_id!: AccountId",
    created_at,
    updated_at
from school
where account_id = $1
"#,
        account_id as &AccountId
    )
    .fetch_optional(pool)
    .await?;

    match record {
        Some(record) => {
            let school = School {
                id: record.id,
                school_name: get_school_name(pool, &record.name).await?.unwrap(),
                location: record.location,
                email: record.email,
                description: record.description,
                profile_image: record.profile_image,
                website: record.website,
                organization_type: record.organization_type,
                account_id: record.account_id,
                created_at: record.created_at,
                updated_at: record.updated_at,
            };

            Ok(Some(school))
        }
        None => Ok(None),
    }
}

#[instrument(skip(pool))]
pub async fn get_account_users_by_account_id(
    pool: &PgPool,
    account_id: &AccountId,
) -> anyhow::Result<Vec<AccountUser>> {
    let records = sqlx::query!(
        // language=SQL
        r#"
select
    user_id as "user_id!: UserId",
    subscription_tier as "subscription_tier?: SubscriptionTier",
    admin as "is_admin!",
    verified as "verified!"
from user_account
where account_id = $1
"#,
        account_id as &AccountId,
    )
    .fetch_all(pool)
    .await?;

    // TODO can probably join these calls to get_profile
    let mut account_users = vec![];
    for record in records {
        let user_profile = db::user::get_profile(pool, &record.user_id).await?.unwrap();

        account_users.push(AccountUser {
            user: user_profile,
            subscription_tier: record.subscription_tier,
            is_admin: record.is_admin,
            verified: record.verified,
        });
    }

    Ok(account_users)
}

#[instrument(skip(pool))]
pub async fn get_school_account_by_id(
    pool: &PgPool,
    school_id: &SchoolId,
) -> sqlx::Result<Option<School>> {
    let record = sqlx::query!(
        // language=SQL
        r#"
select
    school_id as "id!: SchoolId",
    school_name_id as "name!: SchoolNameId",
    location as "location?: serde_json::Value",
    email::text as "email!",
    description,
    profile_image_id as "profile_image?: ImageId",
    website,
    organization_type,
    account_id as "account_id!: AccountId",
    created_at,
    updated_at
from school
where school_id = $1
"#,
        school_id as &SchoolId
    )
    .fetch_optional(pool)
    .await?;

    match record {
        Some(record) => {
            let school = School {
                id: record.id,
                school_name: get_school_name(pool, &record.name).await?.unwrap(),
                location: record.location,
                email: record.email,
                description: record.description,
                profile_image: record.profile_image,
                website: record.website,
                organization_type: record.organization_type,
                account_id: record.account_id,
                created_at: record.created_at,
                updated_at: record.updated_at,
            };

            Ok(Some(school))
        }
        None => Ok(None),
    }
}

pub async fn get_school_name(
    pool: &PgPool,
    school_name_id: &SchoolNameId,
) -> sqlx::Result<Option<SchoolName>> {
    sqlx::query_as!(
        SchoolName,
        // language=SQL
        r#"
select
    school_name_id as "id!: SchoolNameId",
    name::text as "name!",
    verified
from school_name
where school_name_id = $1
"#,
        school_name_id as &SchoolNameId
    )
    .fetch_optional(pool)
    .await
}

#[instrument(skip(pool))]
pub async fn find_school_names_with_schools(
    pool: &PgPool,
    params: &SearchSchoolNamesParams,
) -> sqlx::Result<Vec<(SchoolName, Option<School>)>> {
    let rows = sqlx::query!(
        // language=SQL
        r#"
select
    school_name_id as "id!: SchoolNameId",
    school_name.name::text as "school_name!",
    school_name.verified as "verified!",
    school_id as "school_id?: SchoolId",
    location as "location?: serde_json::Value",
    email::text as "email?",
    description,
    profile_image_id as "profile_image?: ImageId",
    website,
    organization_type,
    account_id as "account_id?: AccountId",
    school.created_at as "created_at?",
    school.updated_at
from school_name
left join school using (school_name_id)
where
    (
        (not $1::bool is null and (verified = $1::bool))
        or $1::bool is null
    )
    and (
        (not $2::text is null and (school_name.name like ('%' || $2::text || '%')::citext))
        or $2::text is null
    )
order by school_name.name asc
limit $3
offset $4
"#,
        params.verified as Option<bool>,
        params.q,
        i64::from(params.page_limit),
        params.page_limit.offset(params.page),
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|record| {
            let school_name = SchoolName {
                id: record.id,
                name: record.school_name,
                verified: record.verified,
            };

            let school = record.school_id.map(|school_id| School {
                id: school_id,
                school_name: school_name.clone(),
                location: record.location,
                email: record.email.unwrap(),
                description: record.description,
                profile_image: record.profile_image,
                website: record.website,
                organization_type: record.organization_type,
                account_id: record.account_id.unwrap(),
                created_at: record.created_at.unwrap(),
                updated_at: record.updated_at,
            });
            (school_name, school)
        })
        .collect())
}

#[instrument(skip(pool))]
pub async fn find_school_names_with_schools_count(
    pool: &PgPool,
    params: &SearchSchoolNamesParams,
) -> sqlx::Result<ItemCount> {
    let rows = sqlx::query_scalar!(
        // language=SQL
        r#"
select
    count(*) as "total_schools!"
from school_name
where
    (
        (not $1::bool is null and (verified = $1::bool))
        or $1::bool is null
    )
    and (
        (not $2::text is null and (school_name.name like ('%' || $2::text || '%')::citext))
        or $2::text is null
    )
"#,
        params.verified as Option<bool>,
        params.q,
    )
    .fetch_one(pool)
    .await?;

    Ok((rows as usize).into())
}

#[instrument(skip(pool))]
pub async fn verify_school_name(
    pool: &PgPool,
    school_name_id: SchoolNameId,
    verified: bool,
) -> sqlx::Result<()> {
    sqlx::query!(
        // language=SQL
        r#"update school_name set verified = $2 where school_name_id = $1"#,
        school_name_id as SchoolNameId,
        verified,
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AccountMember {
    Admin,
    User,
}

pub async fn user_account_membership(
    pool: &PgPool,
    user_id: &UserId,
    account_id: &AccountId,
) -> sqlx::Result<Option<AccountMember>> {
    let res = sqlx::query_scalar!(
        // language=SQL
        r#"select admin from user_account where user_id = $1 and account_id = $2"#,
        user_id as &UserId,
        account_id as &AccountId,
    )
    .fetch_optional(pool)
    .await?;

    Ok(res.map(|admin| {
        if admin {
            AccountMember::Admin
        } else {
            AccountMember::User
        }
    }))
}
