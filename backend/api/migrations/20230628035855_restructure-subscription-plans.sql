-- `if exists` for local dev and re-running migrations.
alter table subscription_plan
    drop constraint if exists subscription_plan_product_id_price_id_key,
    drop column if exists product_id,
    drop column if exists subscription_tier,
    drop column if exists subscription_type,
    drop column if exists billing_interval,
    drop column if exists account_limit,
    drop column if exists amount_in_cents,
    drop column if exists trial_period,
    add column if not exists plan_type smallint not null,
    drop constraint if exists subscription_plan_subscription_type_plan_type,
    add constraint subscription_plan_subscription_type_plan_type unique (plan_type)
;

alter table subscription
    drop column if exists subscription_tier
;
