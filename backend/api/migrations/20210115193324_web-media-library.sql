-- unused to this point, so it's easier to just replace it
drop table web_image_library;

create table web_media_library
(
    id         uuid primary key not null default uuid_generate_v1mc(),
    "hash"     bytea     unique not null,
    kind       int2             not null,
    created_at timestamptz      not null default now(),
    updated_at timestamptz
);

create table web_media_library_url 
(
    media_url  text unique not null,
    media_id   uuid        not null references web_media_library(id) on delete cascade,
    created_at timestamptz not null default now()
);
