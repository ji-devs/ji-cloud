use chrono::{DateTime, Utc};
use shared::domain::billing::{
    AccountId, AmountInCents, CreateSubscriptionRecord, PlanId, PlanType, StripeInvoiceId,
    StripePriceId, StripeSubscriptionId, Subscription, SubscriptionId, SubscriptionPlan,
    SubscriptionStatus, UpdateSubscriptionRecord,
};
use sqlx::PgPool;
use tracing::{instrument, Instrument};

#[instrument(skip(pool))]
pub async fn upsert_subscription_plan(
    pool: &PgPool,
    plan_type: PlanType,
    price_id: StripePriceId,
) -> sqlx::Result<()> {
    sqlx::query!(
        //language=SQL
        r#"
insert into subscription_plan
    (plan_type, price_id)
values
    ($1, $2)
on conflict (plan_type) do update
set
    plan_type = $1,
    price_id = $2,
    updated_at = now()
"#,
        plan_type as PlanType,
        price_id as StripePriceId,
    )
    .execute(pool)
    .instrument(tracing::info_span!("upsert subscription_plan"))
    .await?;

    Ok(())
}

#[instrument(skip(pool))]
pub async fn get_subscription_plan_by_id(
    pool: &PgPool,
    plan_id: PlanId,
) -> sqlx::Result<Option<SubscriptionPlan>> {
    sqlx::query_as!(
        SubscriptionPlan,
        //language=SQL
        r#"
select
    plan_id as "plan_id: PlanId",
    plan_type as "plan_type: PlanType",
    price_id as "price_id: StripePriceId",
    created_at as "created_at: DateTime<Utc>",
    updated_at as "updated_at: DateTime<Utc>"
from subscription_plan
where plan_id = $1
"#,
        uuid::Uuid::from(plan_id),
    )
    .fetch_optional(pool)
    .await
}

#[instrument(skip(pool))]
pub async fn get_subscription_plan_by_type(
    pool: &PgPool,
    plan_type: PlanType,
) -> sqlx::Result<Option<SubscriptionPlan>> {
    sqlx::query_as!(
        SubscriptionPlan,
        //language=SQL
        r#"
select
    plan_id as "plan_id: PlanId",
    plan_type as "plan_type: PlanType",
    price_id as "price_id: StripePriceId",
    created_at as "created_at: DateTime<Utc>",
    updated_at as "updated_at: DateTime<Utc>"
from subscription_plan
where plan_type = $1
"#,
        plan_type as PlanType,
    )
    .fetch_optional(pool)
    .await
}

#[instrument(skip(pool))]
pub async fn create_subscription(
    pool: &PgPool,
    subscription: CreateSubscriptionRecord,
) -> sqlx::Result<SubscriptionId> {
    sqlx::query!(
        //language=SQL
        r#"
insert into subscription
    (
        stripe_subscription_id,
        subscription_plan_id,
        status,
        current_period_end,
        account_id,
        latest_invoice_id,
        amount_due
    )
values
    ($1, $2, $3, $4, $5, $6, $7)
returning subscription_id as "id!: SubscriptionId"
"#,
        subscription.stripe_subscription_id as StripeSubscriptionId,
        subscription.subscription_plan_id as PlanId,
        subscription.status as SubscriptionStatus,
        subscription.current_period_end,
        subscription.account_id as AccountId,
        subscription
            .latest_invoice_id
            .map(|invoice_id| invoice_id.inner()),
        subscription.amount_due_in_cents.map(|due| due.inner()),
    )
    .fetch_one(pool)
    .await
    .map(|res| res.id)
}

#[instrument(skip(pool))]
pub async fn save_subscription(
    pool: &PgPool,
    subscription: UpdateSubscriptionRecord,
) -> sqlx::Result<()> {
    sqlx::query!(
        //language=SQL
        r#"
update subscription
set
    status = coalesce($2, status),
    current_period_end = coalesce($3, current_period_end),
    updated_at = now(),
    latest_invoice_id = case when $4 then $5 else latest_invoice_id end,
    is_trial = $6
where stripe_subscription_id = $1
"#,
        subscription.stripe_subscription_id as StripeSubscriptionId,
        subscription
            .status
            .into_option()
            .map(|status| status as i16),
        subscription.current_period_end.into_option(),
        subscription.latest_invoice_id.is_change(),
        subscription
            .latest_invoice_id
            .into_option()
            .map(|invoice_id| invoice_id.inner()),
        subscription.is_trial.into_option(),
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[instrument(skip(pool))]
pub async fn set_subscription_amount_due(
    pool: &PgPool,
    subscription_id: StripeSubscriptionId,
    amount_due: AmountInCents,
) -> sqlx::Result<()> {
    sqlx::query!(
        //language=SQL
        r#"
update subscription
set
    amount_due = $2
where stripe_subscription_id = $1
"#,
        subscription_id as StripeSubscriptionId,
        amount_due as AmountInCents,
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[instrument(skip(pool))]
pub async fn get_subscription(
    pool: &PgPool,
    subscription_id: SubscriptionId,
) -> sqlx::Result<Option<Subscription>> {
    let row = sqlx::query!(
        // language=SQL
        r#"
select
    subscription_id as "subscription_id!: SubscriptionId",
    stripe_subscription_id as "stripe_subscription_id!: StripeSubscriptionId",
    subscription_plan.plan_type as "subscription_plan_type!: PlanType",
    is_trial,
    status as "status!: SubscriptionStatus",
    current_period_end as "current_period_end!: DateTime<Utc>",
    account_id as "account_id!: AccountId",
    latest_invoice_id as "latest_invoice_id?: StripeInvoiceId",
    amount_due as "amount_due_in_cents?: AmountInCents",
    subscription.created_at as "created_at!: DateTime<Utc>",
    subscription.updated_at as "updated_at?: DateTime<Utc>"
from subscription
inner join subscription_plan on subscription.subscription_plan_id = subscription_plan.plan_id
where subscription_id = $1
"#,
        subscription_id as SubscriptionId,
    )
    .fetch_optional(pool)
    .await?;

    let subscription = row.map(|row| Subscription {
        subscription_id: row.subscription_id,
        stripe_subscription_id: row.stripe_subscription_id,
        subscription_plan_type: row.subscription_plan_type,
        auto_renew: row.status.is_active(),
        status: row.status,
        is_trial: row.is_trial,
        current_period_end: row.current_period_end,
        account_id: row.account_id,
        latest_invoice_id: row.latest_invoice_id,
        amount_due_in_cents: row.amount_due_in_cents,
        created_at: row.created_at,
        updated_at: row.updated_at,
    });

    Ok(subscription)
}

#[instrument(skip(pool))]
pub async fn get_latest_subscription_by_account_id(
    pool: &PgPool,
    account_id: AccountId,
) -> sqlx::Result<Option<Subscription>> {
    let row = sqlx::query!(
        // language=SQL
        r#"
select
    subscription_id as "subscription_id!: SubscriptionId",
    stripe_subscription_id as "stripe_subscription_id!: StripeSubscriptionId",
    subscription_plan.plan_type as "subscription_plan_type!: PlanType",
    is_trial,
    status as "status!: SubscriptionStatus",
    current_period_end as "current_period_end!: DateTime<Utc>",
    account_id as "account_id!: AccountId",
    latest_invoice_id as "latest_invoice_id?: StripeInvoiceId",
    amount_due as "amount_due_in_cents?: AmountInCents",
    subscription.created_at as "created_at!: DateTime<Utc>",
    subscription.updated_at as "updated_at?: DateTime<Utc>"
from subscription
inner join subscription_plan on subscription.subscription_plan_id = subscription_plan.plan_id
where account_id = $1
order by subscription.created_at desc
limit 1
"#,
        account_id as AccountId,
    )
    .fetch_optional(pool)
    .await?;

    let subscription = row.map(|row| Subscription {
        subscription_id: row.subscription_id,
        stripe_subscription_id: row.stripe_subscription_id,
        subscription_plan_type: row.subscription_plan_type,
        auto_renew: row.status.is_active(),
        status: row.status,
        is_trial: row.is_trial,
        current_period_end: row.current_period_end,
        account_id: row.account_id,
        latest_invoice_id: row.latest_invoice_id,
        amount_due_in_cents: row.amount_due_in_cents,
        created_at: row.created_at,
        updated_at: row.updated_at,
    });

    Ok(subscription)
}

#[instrument(skip(pool))]
pub async fn get_stripe_subscription_id_by_invoice_id(
    pool: &PgPool,
    invoice_id: &StripeInvoiceId,
) -> sqlx::Result<Option<StripeSubscriptionId>> {
    sqlx::query_scalar!(
        // language=SQL
        r#"select stripe_subscription_id as "id: StripeSubscriptionId" from subscription where latest_invoice_id = $1"#,
        invoice_id as &StripeInvoiceId,
    )
    .fetch_optional(pool)
    .await
}
