use crate::db;
use shared::domain::billing::{
    Account, AccountId, AccountType, CustomerId, PaymentMethod, School, SchoolId, SchoolName,
    SchoolNameId, SubscriptionStatus, SubscriptionTier, UserAccountSummary,
};
use shared::domain::image::ImageId;
use shared::domain::user::UserId;
use sqlx::PgPool;
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
    new_name: String,
    verified: bool,
) -> sqlx::Result<SchoolNameId> {
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
    sqlx::query!(
        // language=SQL
        r#"
insert into user_account
(user_id, account_id, subscription_tier, admin, verified)
values
($1, $2, $3, true, true)"#,
        user_id as &UserId,
        account_id as AccountId,
        subscription_tier as &SubscriptionTier,
    )
    .execute(&mut txn)
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
    sqlx::query!(
        // language=SQL
        r#"
insert into user_account
(user_id, account_id, subscription_tier, admin, verified)
values
($1, $2, $3, true, true)"#,
        user_id as UserId,
        account_id as AccountId,
        SubscriptionTier::Pro as SubscriptionTier,
    )
    .execute(&mut txn)
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
    db: &sqlx::Pool<sqlx::Postgres>,
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
    db: &sqlx::Pool<sqlx::Postgres>,
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
    db: &sqlx::Pool<sqlx::Postgres>,
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
                name: get_school_name(pool, &record.name).await?.unwrap(),
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
pub async fn get_school_names_with_schools(
    pool: &PgPool,
    verified: Option<bool>,
) -> sqlx::Result<Vec<(SchoolName, Option<School>)>> {
    let rows = sqlx::query!(
        // language=SQL
        r#"
select
    school_name_id as "id!: SchoolNameId",
    school_name.name::text as "school_name!",
    school_name.verified,
    school_id as "school_id?: SchoolId",
    location as "location?: serde_json::Value",
    email::text as "email?",
    description,
    profile_image_id as "profile_image?: ImageId",
    website,
    organization_type,
    account_id as "account_id?: AccountId",
    school.created_at,
    school.updated_at
from school_name
left join school using (school_name_id)
where
    (not $1::bool is null and (verified = $1::bool))
    or $1::bool is null
"#,
        verified as Option<bool>,
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
                name: school_name.clone(),
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
