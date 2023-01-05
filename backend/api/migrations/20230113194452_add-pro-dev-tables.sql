-- Holds ProDev metadata
create table pro_dev_data
(
    id                     uuid                     default uuid_generate_v1mc() not null
            primary key,
    draft_or_live          smallint,
    display_name           text                     default ''::text             not null,
    language               text                     default ''::text             not null,
    description            text                     default ''::text             not null,
    privacy_level          smallint                 default 1                    not null,
    other_keywords         text                     default ''::text             not null,
    translated_keywords    text                     default ''::text             not null,
    duration               bigint,
    translated_name        jsonb                    default '{}'::jsonb          not null,
    translated_description jsonb                    default '{}'::jsonb          not null,
    last_synced_at         timestamp with time zone,
    created_at             timestamp with time zone default now()                not null,
    updated_at             timestamp with time zone
);

-- Holds ProDev data
create table pro_dev
(
    id              uuid    default uuid_generate_v1mc() not null
            primary key,
    creator_id      uuid
            references "user" (id)
            on delete cascade,
    author_id       uuid
            references "user" (id)
            on delete cascade,
    live_id         uuid                                 not null
            references pro_dev_data (id)
            on delete restrict,
    draft_id        uuid                                 not null
            references pro_dev_data (id)
            on delete restrict,
    published_at    timestamp with time zone,
    likes           bigint  default 0                    not null
            check (likes >= 0),
    plays           bigint  default 0                    not null
            check (plays >= 0),
    live_up_to_date boolean default false                not null,

    constraint pro_dev_check
        check (live_id <> draft_id)
);

create index pro_dev_draft_id
    on pro_dev (draft_id);

create index pro_dev_live_id
    on pro_dev (live_id);

-- Holds ProDev categories
create table pro_dev_data_category
(
    pro_dev_data_id     uuid                            not null
            references pro_dev_data (id)
            on delete cascade,
    category_id         uuid                            not null
            references category (id),
    unique (pro_dev_data_id, category_id)
);

create index pro_dev_data_category_pro_dev_data_id_idx
    on pro_dev_data_category (pro_dev_data_id);

-- Holds ProDev additional resources
create table pro_dev_data_resource
(
    id               uuid  default uuid_generate_v1mc() not null
            primary key,
    pro_dev_data_id   uuid                               not null
        references pro_dev_data
            on delete cascade,
    display_name     text                               not null,
    resource_type_id uuid                               not null
        references resource_type
            on delete cascade,
    resource_content jsonb default '{}'::jsonb          not null,
    unique (pro_dev_data_id, resource_content, resource_type_id)
);

create index pro_dev_data_resource_pro_dev_data_id_idx
    on pro_dev_data_resource (pro_dev_data_id);

-- Holds ProDev Covers
create table pro_dev_data_module
(
    id             uuid                     default uuid_generate_v1mc() not null
        primary key,
    index          smallint                                              not null
        check (index = 0),
    pro_dev_data_id uuid                                                  not null
        references pro_dev_data (id)
            on delete cascade,
    kind           smallint                                              not null,
    is_complete    boolean                  default false                not null,
    contents       jsonb                    default '{}'::jsonb          not null,
    created_at     timestamp with time zone default now()                not null,
    updated_at     timestamp with time zone default now()                not null,
    unique (index, pro_dev_data_id)
);

create index pro_dev_data_module_pro_dev_data_id_idx
    on pro_dev_data_module (pro_dev_data_id);

-- Holds ProDev units
create table pro_dev_data_unit
(
    id           uuid                     default uuid_generate_v1mc()    not null
            primary key,
    pro_dev_data_id uuid not null
            references pro_dev_data (id)
            on delete cascade,
    display_name    text                           default ''::text       not null,
    description     text                           default ''::text       not null,
    created_at      timestamp with time zone       default now()          not null,
    updated_at      timestamp with time zone,
    index           smallint
            check (index >= 0),
    value        jsonb   default '{}'::jsonb                              not null,

    unique (pro_dev_data_id, id, index)

);

create index pro_dev_data_unit_pro_dev_data_id_idx
    on pro_dev_data_unit (pro_dev_data_id);
