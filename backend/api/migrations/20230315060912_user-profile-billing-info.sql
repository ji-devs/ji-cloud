create table subscription
(
    subscription_id uuid primary key default uuid_generate_v1mc(),
    stripe_subscription_id text unique not null,
    subscription_plan_id uuid references "subscription_plan" (plan_id) not null,
    subscription_tier int2 not null,
    auto_renew boolean not null,
    status int2 not null default 0,
    current_period_end timestamptz not null,
    user_id uuid references "user" (id),
    latest_invoice_id text,
    amount_due int8,
    created_at timestamptz not null default now(),
    updated_at timestamptz
);

alter table user_profile
    add column if not exists stripe_customer_id text,
    add column if not exists payment_method jsonb,
    add column if not exists subscription_id uuid references "subscription" (subscription_id)
;

create index stripe_customer_idx on user_profile (stripe_customer_id) where stripe_customer_id is not null;
