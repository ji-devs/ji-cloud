-- we have to truncate the users table, due to the migration being incompatible (can't have default values for these columns)
truncate "user" cascade;

alter table "user"
    rename column display_name to username;

alter table "user"
    add unique (username),
    add column over_18                boolean not null,
    add column given_name             text    not null,
    add column family_name            text    not null,
    add column language               text    not null,
    add column locale                 text    not null,
    add column timezone               text    not null,
    add column organization           text    not null,
    add column email_verified         bool    not null default false,
    add column opt_into_edu_resources boolean not null;
-- add column `location_*`? or just `location`

create table "subject"
(
    subject_id   uuid primary key not null default uuid_generate_v1mc(),
    display_name text             not null,
    created_at   timestamptz      not null,
    updated_at   timestamptz
);

create table user_subject
(
    user_id    uuid references "user" (id)          not null,
    subject_id uuid references "subject" (subject_id) not null,
    created_at timestamptz                          not null default now(),
    unique (user_id, subject_id)
);

create table user_affiliation
(
    user_id        uuid references "user" (id) not null,
    affiliation_id uuid references affiliation (id),
    created_at     timestamptz                 not null default now(),
    unique (user_id, affiliation_id)
);

create table user_age_range
(
    user_id      uuid references "user" (id) not null,
    age_range_id uuid references age_range (id),
    created_at   timestamptz                 not null default now(),
    unique (user_id, age_range_id)
);
