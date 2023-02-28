create table "subscription_plan"
(
    plan_id uuid primary key not null default uuid_generate_v1mc(),
    product_id text not null,
    price_id text not null,
    subscription_tier int2 not null,
    subscription_type int2 not null,
    billing_interval int2 not null,
    account_limit int8,
    amount_in_cents int8 not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz,
    unique(product_id, price_id)
);
