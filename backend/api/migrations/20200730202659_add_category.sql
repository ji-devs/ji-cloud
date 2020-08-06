create table "category"
(
    id         uuid primary key     default uuid_generate_v1mc(),
    parent_id  uuid references "category" (id),
    name       text        not null,
    "index"    int2        not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz
);

