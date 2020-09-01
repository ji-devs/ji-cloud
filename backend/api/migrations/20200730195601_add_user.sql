create table "user"
(
    id                     uuid primary key       default uuid_generate_v1mc(),
    firebase_id            text unique   not null,
    username               text unique   not null,
    email                  citext unique not null,
    over_18                boolean       not null,
    given_name             text          not null,
    family_name            text          not null,
    language               text          not null,
    locale                 text          not null,
    timezone               text          not null,
    email_verified         bool          not null default false,
    opt_into_edu_resources boolean       not null,
    created_at             timestamptz   not null default now(),
    updated_at             timestamptz
);
