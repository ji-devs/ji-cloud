create table style
(
    id           uuid primary key     default uuid_generate_v1mc(),
    display_name text        not null,
    created_at   timestamptz not null default now(),
    updated_at   timestamptz
);

create table age_range
(
    id           uuid primary key     default uuid_generate_v1mc(),
    display_name text        not null,
    created_at   timestamptz not null default now(),
    updated_at   timestamptz
);

create table affiliation
(
    id           uuid primary key     default uuid_generate_v1mc(),
    display_name text        not null,
    created_at   timestamptz not null default now(),
    updated_at   timestamptz
);

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
    style_id   uuid        not null references style (id),
    created_at timestamptz not null default now(),
    unique (image_id, style_id)
);

create table image_age_range
(
    image_id     uuid        not null references image_metadata (id),
    age_range_id uuid        not null references age_range (id),
    created_at   timestamptz not null default now(),
    unique (image_id, age_range_id)
);

create table image_affiliation
(
    image_id       uuid        not null references image_metadata (id),
    affiliation_id uuid        not null references affiliation (id),
    created_at     timestamptz not null default now(),
    unique (image_id, affiliation_id)

);

create table image_categories
(
    image_id    uuid        not null references image_metadata (id),
    category_id uuid        not null references category (id),
    created_at  timestamptz not null default now(),
    unique (image_id, category_id)
);
