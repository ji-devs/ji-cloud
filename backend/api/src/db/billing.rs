use chrono::{DateTime, Utc};
use shared::domain::billing::{
    AccountLimit, AmountInCents, BillingInterval, PlanId, StripePriceId, StripeProductId,
    SubscriptionPlan, SubscriptionTier, SubscriptionType,
};
use sqlx::PgPool;
use tracing::{instrument, Instrument};

#[instrument(skip(pool))]
pub async fn upsert_subscription_plan(
    pool: &PgPool,
    mut plan: SubscriptionPlan,
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

#[instrument(skip(txn))]
pub async fn get_subscription_plans(
    txn: &mut sqlx::Transaction<'_, sqlx::Postgres>,
) -> sqlx::Result<Vec<SubscriptionPlan>> {
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
    .fetch_all(&mut *txn)
    .instrument(tracing::info_span!("get subscription plans"))
    .await
}
