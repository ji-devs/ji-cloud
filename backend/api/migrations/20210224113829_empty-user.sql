create table "user_profile" (
    user_id                uuid        primary key references "user" (id),
    username               text        not null,
    over_18                boolean     not null,
    given_name             text        not null,
    family_name            text        not null,
    language               text        not null,
    locale                 text        not null,
    timezone               text        not null,
    organization           text,
    opt_into_edu_resources boolean     not null,
    geocode                text,
    location               jsonb,
    created_at             timestamptz not null    default now(),
    updated_at             timestamptz
);

insert into user_profile (
    user_id,
    username,
    over_18,
    given_name,
    family_name,
    language,
    locale,
    timezone,
    organization,
    opt_into_edu_resources,
    geocode,
    location,
    created_at,
    updated_at
)
select id,
       username,
       over_18,
       given_name,
       family_name,
       language,
       locale,
       timezone,
       organization,
       opt_into_edu_resources,
       geocode,
       location,
       created_at,
       updated_at
from "user";

alter table "user"
    drop over_18,
    drop username,
    drop given_name,
    drop family_name,
    drop language,
    drop locale,
    drop timezone,
    drop organization,
    drop opt_into_edu_resources,
    drop geocode,
    drop location;

create table user_email (
    user_id    uuid          primary key references "user" (id),
    email      citext        unique      not null,
    created_at timestamptz               not null default now(),
    updated_at timestamptz
);

insert into user_email (user_id, email, created_at)
select id, email, created_at from "user";

alter table "user" drop email, drop email_verified_at;
