use chrono::{DateTime, Utc};
use shared::domain::billing::{
    AccountLimit, AmountInCents, BillingInterval, CreateSubscriptionRecord,
    CreateUpdateSubscriptionPlanRequest, PlanId, StripePriceId, StripeProductId,
    StripeSubscriptionId, Subscription, SubscriptionId, SubscriptionPlan, SubscriptionStatus,
    SubscriptionTier, SubscriptionType, UpdateSubscriptionRecord,
};
use shared::domain::user::UserId;
use sqlx::PgPool;
use tracing::{instrument, Instrument};

#[instrument(skip(pool))]
pub async fn upsert_subscription_plan(
    pool: &PgPool,
    mut plan: CreateUpdateSubscriptionPlanRequest,
) -> sqlx::Result<()> {
    if let SubscriptionType::Individual = &plan.subscription_type {
        // Always enforce this.
        plan.account_limit = Some(AccountLimit::from(1));
    }

    sqlx::query!(
        //language=SQL
        r#"
insert into subscription_plan
    (product_id, price_id, subscription_tier, subscription_type, billing_interval, account_limit, amount_in_cents)
values
    ($1, $2, $3, $4, $5, $6, $7)
on conflict (product_id, price_id) do update
set
    product_id = $1,
    price_id = $2,
    subscription_tier = $3,
    subscription_type = $4,
    billing_interval = $5,
    account_limit = $6,
    amount_in_cents = $7
"#,
        plan.product_id as StripeProductId,
        plan.price_id as StripePriceId,
        plan.subscription_tier as SubscriptionTier,
        plan.subscription_type as SubscriptionType,
        plan.billing_interval as BillingInterval,
        plan.account_limit as Option<AccountLimit>,
        plan.amount_in_cents as AmountInCents,
    )
    .execute(&*pool)
    .instrument(tracing::info_span!("upsert subscription_plan"))
    .await?;

    Ok(())
}

#[instrument(skip(pool))]
pub async fn get_subscription_plans(pool: &PgPool) -> sqlx::Result<Vec<SubscriptionPlan>> {
    sqlx::query_as!(
        SubscriptionPlan,
        //language=SQL
        r#"
select
    plan_id as "plan_id: PlanId",
    product_id as "product_id: StripeProductId",
    price_id as "price_id: StripePriceId",
    subscription_tier as "subscription_tier: SubscriptionTier",
    subscription_type as "subscription_type: SubscriptionType",
    billing_interval as "billing_interval: BillingInterval",
    account_limit as "account_limit: AccountLimit",
    amount_in_cents as "amount_in_cents: AmountInCents",
    created_at as "created_at: DateTime<Utc>",
    updated_at as "updated_at: DateTime<Utc>"
from subscription_plan
"#
    )
    .fetch_all(&*pool)
    .instrument(tracing::info_span!("get subscription plans"))
    .await
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
    product_id as "product_id: StripeProductId",
    price_id as "price_id: StripePriceId",
    subscription_tier as "subscription_tier: SubscriptionTier",
    subscription_type as "subscription_type: SubscriptionType",
    billing_interval as "billing_interval: BillingInterval",
    account_limit as "account_limit: AccountLimit",
    amount_in_cents as "amount_in_cents: AmountInCents",
    created_at as "created_at: DateTime<Utc>",
    updated_at as "updated_at: DateTime<Utc>"
from subscription_plan
where plan_id = $1
"#,
        uuid::Uuid::from(plan_id),
    )
    .fetch_optional(&*pool)
    .instrument(tracing::info_span!("get subscription plans"))
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
        subscription_tier,
        auto_renew,
        status,
        current_period_end,
        user_id
    )
values
    ($1, $2, $3, $4, $5, $6, $7)
returning subscription_id as "id!: SubscriptionId"
"#,
        subscription.stripe_subscription_id as StripeSubscriptionId,
        subscription.subscription_plan_id as PlanId,
        subscription.tier as SubscriptionTier,
        subscription.auto_renew,
        subscription.status as SubscriptionStatus,
        subscription.current_period_end,
        subscription.user_id as UserId,
    )
    .fetch_one(&*pool)
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
    auto_renew = coalesce($2, auto_renew),
    status = coalesce($3, status),
    current_period_end = coalesce($4, current_period_end),
    updated_at = now()
where stripe_subscription_id = $1
"#,
        subscription.stripe_subscription_id as StripeSubscriptionId,
        subscription.auto_renew,
        subscription.status.map(|status| status as i16),
        subscription.current_period_end,
    )
    .execute(&*pool)
    .await?;

    Ok(())
}

#[instrument()]
pub async fn get_subscription(
    pool: &PgPool,
    subscription_id: SubscriptionId,
) -> sqlx::Result<Option<Subscription>> {
    sqlx::query_as!(
        Subscription,
        // language=SQL
        r#"
select
    subscription_id as "subscription_id!: SubscriptionId",
    stripe_subscription_id as "stripe_subscription_id!: StripeSubscriptionId",
    subscription_plan_id as "subscription_plan_id!: PlanId",
    subscription_tier as "tier!: SubscriptionTier",
    auto_renew,
    status as "status!: SubscriptionStatus",
    current_period_end as "current_period_end!: DateTime<Utc>",
    user_id as "user_id!: UserId",
    created_at as "created_at!: DateTime<Utc>",
    updated_at as "updated_at?: DateTime<Utc>"
from subscription
where subscription_id = $1
"#,
        subscription_id as SubscriptionId,
    )
    .fetch_optional(&*pool)
    .await
}
