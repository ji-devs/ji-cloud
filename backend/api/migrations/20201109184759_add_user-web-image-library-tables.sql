create table user_image_library
(
    id         uuid primary key not null default uuid_generate_v1mc(),
    user_id    uuid references "user" (id) on delete cascade,
    created_at timestamptz      not null,
    updated_at timestamptz
);

create table web_image_library
(
    id         uuid primary key not null default uuid_generate_v1mc(),
    "hash"     bytea            not null,
    url        text unique      not null,
    created_at timestamptz      not null,
    updated_at timestamptz
);
