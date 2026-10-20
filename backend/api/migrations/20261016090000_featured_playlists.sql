create table featured_playlists
(
    id          uuid primary key default uuid_generate_v1mc(),
    playlist_id uuid not null references playlist (id) on delete cascade,
    index       int not null,
    created_at  timestamptz not null default now(),
    updated_at  timestamptz
);
