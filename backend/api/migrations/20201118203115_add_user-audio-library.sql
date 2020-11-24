create table user_audio_library
(
    id         uuid primary key not null default uuid_generate_v1mc(),
    user_id    uuid references "user" (id) on delete cascade,
    created_at timestamptz      not null,
    updated_at timestamptz
);
