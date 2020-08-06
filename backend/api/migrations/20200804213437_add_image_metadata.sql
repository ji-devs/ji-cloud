create table image_metadata
(
    id          uuid primary key     default uuid_generate_v1mc(),
    source      text        not null,
    name        text        not null,
    description text        not null,
    is_premium  bool        not null,
    created_at  timestamptz not null default now(),
    updated_at  timestamptz,
    publish_at  timestamptz
);

create table image_style
(
    image_id   uuid        not null references image_metadata (id),
    style      int2        not null,
    created_at timestamptz not null default now(),
    unique (image_id, style)
);

create table image_age_range
(
    image_id   uuid        not null references image_metadata (id),
    age_range  int2        not null,
    created_at timestamptz not null default now(),
    unique (image_id, age_range)

);

create table image_affiliation
(
    image_id    uuid        not null references image_metadata (id),
    affiliation int2        not null,
    created_at  timestamptz not null default now(),
    unique (image_id, affiliation)

);

create table image_categories
(
    image_id    uuid        not null references image_metadata (id),
    category_id uuid        not null references category (id),
    created_at  timestamptz not null default now(),
    unique (image_id, category_id)
);
