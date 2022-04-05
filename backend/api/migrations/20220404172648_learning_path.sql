create table learning_path_data 
(
    id                      uuid                     default uuid_generate_v1mc() not null    
        primary key,
    draft_or_live           smallint,
    display_name            text                     default ''::text             not null,
    language                text                                                  not null,
    description             text                     default ''::text             not null,
    privacy_level           smallint                 default 1                    not null,
    other_keywords          text                     default ''::text             not null,
    translated_keywords     text                     default ''::text             not null,
    translated_description  jsonb                    default '{}'::jsonb          not null,
    last_synced_at          timestamp with time zone,
    created_at              timestamp with time zone default now()                not null,
    updated_at              timestamp with time zone
);

create table learning_path
(
    id           uuid     default uuid_generate_v1mc() not null
        primary key,
    creator_id   uuid
        references "user" (id)
            on delete cascade,
    author_id    uuid
        references "user" (id)
            on delete cascade,
    live_id      uuid                                  not null
        references learning_path_data (id)
            on delete restrict,
    draft_id     uuid                                  not null
        references learning_path_data (id)
            on delete restrict,
    published_at timestamp with time zone,
    likes  bigint   default 0                    not null
            check (likes >= 0),
    plays  bigint   default 0                    not null
            check (plays >= 0)
    constraint jig_check check (live_id <> draft_id)
);

-- JIGs added to Learning Paths
create table learning_path_data_jig (
    learning_path_data_id   uuid                        not null 
        references learning_path_data(id)
            on delete cascade,
    jig_id    uuid                                      not null
        references "jig" (id)
            on delete cascade, 
    unique (learning_path_data_id, jig_id)
);

create table learning_path_data_resource
(
    id               uuid  default uuid_generate_v1mc() not null
        primary key,
    learning_path_data_id      uuid                               not null
        references learning_path_data (id)
            on delete cascade,
    display_name     text                               not null,
    resource_type_id uuid                               not null
        references resource_type,
    resource_content jsonb default '{}'::jsonb          not null,
        unique (learning_path_data_id, resource_content, resource_type_id)
);

create table learning_path_data_affiliation
(
    learning_path_data_id    uuid not null
        references learning_path_data (id)
            on delete cascade,
    affiliation_id uuid not null
        references affiliation (id), 
        unique (learning_path_data_id, affiliation_id)
);

create table learning_path_data_age_range
(
    learning_path_data_id  uuid not null
        references learning_path_data (id)
            on delete cascade,
    age_range_id uuid not null
        references age_range (id), 
        unique (learning_path_data_id, age_range_id)
);

create table learning_path_data_category
(
    learning_path_data_id uuid not null
        references learning_path_data(id)
            on delete cascade,
    category_id uuid not null
        references category(id)
            on delete cascade,
        unique (learning_path_data_id, category_id)
);

create table learning_path_like
(
    learning_path_id     uuid                                   not null
        references learning_path (id)
            on delete cascade,
    user_id    uuid                                   not null,
    created_at timestamp with time zone default now() not null,
    primary key (learning_path_id, user_id)
);

create index learning_path_draft_id
    on learning_path (draft_id);

create index learning_path_live_id
    on learning_path (live_id);

create index learning_path_data_resource_learning_path_data_id_idx
    on learning_path_data_resource (learning_path_data_id);

create index learning_path_data_affiliation_learning_path_data_id_idx
    on learning_path_data_affiliation (learning_path_data_id);

create index learning_path_data_age_range_learning_path_data_id_idx
    on learning_path_data_age_range (learning_path_data_id);

create index learning_path_data_category_learning_path_data_id_idx
    on learning_path_data_category (learning_path_data_id);

    