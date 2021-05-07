alter table animation
    rename to "animation_metadata";
alter table animation_metadata
    rename constraint animation_pkey to "animation_metadata_pkey";
alter table animation_metadata
    rename column variant to "kind";
alter table animation_metadata add column last_synced_at timestamptz;

select trigger_updated_at('animation_metadata');

create table animation_tag
(
    id           uuid primary key     default uuid_generate_v1mc(),
    display_name text        not null,
    created_at   timestamptz not null default now(),
    updated_at   timestamptz,
    index        int2 unique not null check (index >= 0)
);

select trigger_updated_at('animation_tag');

create table animation_style
(
    animation_id   uuid        not null references animation_metadata (id),
    style_id   uuid        not null references style (id),
    created_at   timestamptz not null default now(),
    unique (style_id, animation_id)
);

select trigger_updated_at('animation_style');

create table audio_metadata
(
    id          uuid        default uuid_generate_v1mc() primary key not null,
    name        text                                                 not null,
    description text                                                 not null,
    is_premium  boolean                                              not null,
    created_at  timestamptz default now()                            not null,
    updated_at  timestamptz,
    publish_at  timestamptz,
    kind        smallint                                             not null,
    uploaded_at timestamptz
);

select trigger_updated_at('audio_metadata');

create table audio_tag
(
    id           uuid primary key default uuid_generate_v1mc(),
    display_name text        not null,
    created_at   timestamptz not null,
    updated_at   timestamptz,
    index        int2 unique not null check (index >= 0)
);

select trigger_updated_at('audio_tag');

create table audio_style
(
    id           uuid primary key default uuid_generate_v1mc(),
    display_name text        not null,
    created_at   timestamptz not null,
    updated_at   timestamptz,
    index        int2 unique not null check (index >= 0)
);

select trigger_updated_at('audio_style');
