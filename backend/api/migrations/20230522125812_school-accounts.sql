-- Migrations
create table "account"
(
    account_id uuid primary key not null default uuid_generate_v1mc(),
    account_type int2 not null,
    stripe_customer_id text,
    payment_method jsonb,
    created_at timestamptz not null default now(),
    updated_at timestamptz
)
;

alter table "subscription"
    drop column if exists user_id,
    add column if not exists account_id uuid not null references "account" (account_id)
;

create table "user_account"
(
    user_id uuid not null references "user" (id),
    account_id uuid not null references "account" (account_id),
    subscription_tier int2,
    admin bool not null default false,
    verified bool not null default false
)
;

drop index if exists stripe_customer_idx;
create index if not exists account_stripe_customer_idx on account (stripe_customer_id) where stripe_customer_id is not null;

alter table user_profile
    drop column if exists stripe_customer_id,
    drop column if exists payment_method,
    drop column if exists subscription_id
;

create table "school_name"
(
    school_name_id uuid primary key not null default uuid_generate_v1mc(),
    name citext not null,
    verified bool not null default false,
    unique(name)
)
;

create table "school"
(
    school_id uuid primary key not null default uuid_generate_v1mc(),
    school_name_id uuid not null references school_name (school_name_id),
    email citext not null,
    location jsonb,
    description text,
    website text,
    organization_type text,
    profile_image_id uuid references user_image_library (id),
    account_id uuid not null references "account" (account_id),
    created_at timestamptz not null default now(),
    updated_at timestamptz,
    unique(school_name_id)
)
;
