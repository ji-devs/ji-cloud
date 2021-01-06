create table animation
(
    id          uuid primary key not null default uuid_generate_v1mc(),
    name        text             not null,
    description text             not null,
    is_premium  bool             not null,
    created_at  timestamptz      not null default now(),
    updated_at  timestamptz,
    looping     bool             not null,
    publish_at  timestamptz,
    variant     int2             not null
);
