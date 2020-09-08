create table "user"
(
    id           uuid primary key       default uuid_generate_v1mc(),
    firebase_id  text unique   not null,
    display_name text          not null,
    email        citext unique not null,
    created_at   timestamptz   not null default now(),
    updated_at   timestamptz
);
