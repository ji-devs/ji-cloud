truncate "user" cascade;

alter table "user" drop column "firebase_id";
alter table "user" drop column "email_verified";
alter table "user" add column email_verified_at timestamptz;

create table "user_auth_google" (
    user_id      uuid primary key references "user" (id) on delete cascade,
    google_id    text unique not null,
    access_token text unique not null,
    expires_at   timestamptz not null,
    created_at   timestamptz not null default now(),
    updated_at   timestamptz
);

create table "user_auth_basic" (
    user_id    uuid primary key references "user" (id) on delete cascade,
    email      citext unique not null,
    password   text          not null,
    created_at timestamptz   not null default now(),
    updated_at timestamptz
);
