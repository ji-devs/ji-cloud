create table resource_data
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
    translated_name         jsonb                    default '{}'::jsonb          not null,
    translated_description  jsonb                    default '{}'::jsonb          not null,
    locked                  bool                     default false                not null,
    last_synced_at          timestamp with time zone,
    created_at              timestamp with time zone default now()                not null,
    updated_at              timestamp with time zone
);

create table "resource"
(
    id           uuid     default uuid_generate_v1mc() not null
        primary key,
    creator_id   uuid
        references "user" (id)
            on delete cascade,
    author_id    uuid
        references "user" (id)
            on delete cascade,
    parents      uuid[]   default '{}'::uuid[]         not null,
    live_id      uuid                                  not null
        references resource_data (id)
            on delete restrict,
    draft_id     uuid                                  not null
        references resource_data (id)
            on delete restrict,
    published_at timestamp with time zone,
    likes  bigint   default 0                    not null
            check (likes >= 0),
    plays  bigint   default 0                    not null
            check (plays >= 0)
    constraint resource_check check (live_id <> draft_id)
);

create table resource_data_module
(
    id          uuid                     default uuid_generate_v1mc() not null
        primary key,
    index       smallint                                              not null
        constraint resource_data_module_index_check
            check (index = 0),
    resource_data_id uuid                                                  not null
        references resource_data (id)
            on delete cascade,
    kind        smallint                                              not null,
    is_complete boolean                  default false                not null,
    contents    jsonb                    default '{}'::jsonb          not null,
    created_at  timestamp with time zone default now()                not null,
    updated_at  timestamp with time zone default now()                not null,
    unique (index, resource_data_id)
        deferrable initially deferred
);

create table resource_data_resource
(
    id               uuid  default uuid_generate_v1mc() not null
        primary key,
    resource_data_id      uuid                               not null
        references resource_data (id)
            on delete cascade,
    display_name     text                               not null,
    resource_type_id uuid                               not null
        references resource_type,
    resource_content jsonb default '{}'::jsonb          not null,
        unique (resource_data_id, resource_content, resource_type_id)
);

create table resource_data_affiliation
(
    resource_data_id    uuid not null
        references resource_data (id)
            on delete cascade,
    affiliation_id uuid not null
        references affiliation (id),
        unique (resource_data_id, affiliation_id)
);

create table resource_data_age_range
(
    resource_data_id  uuid not null
        references resource_data (id)
            on delete cascade,
    age_range_id uuid not null
        references age_range (id),
        unique (resource_data_id, age_range_id)
);

create table resource_data_category
(
    resource_data_id uuid not null
        references resource_data(id)
            on delete cascade,
    category_id uuid not null
        references category(id)
            on delete cascade,
        unique (resource_data_id, category_id)
);

create table resource_like
(
    resource_id     uuid                                   not null
        references resource (id)
            on delete cascade,
    user_id    uuid                                   not null,
    created_at timestamp with time zone default now() not null,
    primary key (resource_id, user_id)
);

create table resource_admin_data
(
    rating  smallint,
    blocked boolean default false not null,
    curated boolean default false not null,
    resource_id  uuid                  not null
        primary key
        references resource (id)
            on delete cascade
);

create table resource_curation_comment
(
    id         uuid                     default uuid_generate_v1mc() not null
        primary key,
    resource_id     uuid                                                  not null
        references resource (id)
            on delete cascade,
    comment    text                                                  not null,
    author_id  uuid                                                  not null
        references "user",
    created_at timestamp with time zone default now()                not null
);

create table resource_curation_data
(
    resource_id               uuid                   not null
        references resource (id)
            on delete cascade,
    display_name         boolean  default false not null,
    language             boolean  default false not null,
    categories           boolean  default false not null,
    description          boolean  default false not null,
    age_ranges           boolean  default false not null,
    affiliations         boolean  default false not null,
    additional_resources boolean  default false not null,
    curation_status      smallint default 0     not null,
    updated_at           timestamp with time zone
);

create table resource_report
(
    id          uuid                     default uuid_generate_v1mc() not null
        primary key,
    resource_id      uuid                                                  not null
        references resource (id)
            on delete cascade,
    report_type smallint                                              not null,
    reporter_id uuid
        references "user",
    created_at  timestamp with time zone default now()                not null
);

create index resource_draft_id
    on resource (draft_id);

create index resource_live_id
    on resource (live_id);

create index resource_data_resource_resource_data_id_idx
    on resource_data_resource (resource_data_id);

create index resource_data_affiliation_resource_data_id_idx
    on resource_data_affiliation (resource_data_id);

create index resource_data_age_range_resource_data_id_idx
    on resource_data_age_range (resource_data_id);

create index resource_data_category_resource_data_id_idx
    on resource_data_category (resource_data_id);

create index resource_data_module_resource_data_id_idx
    on resource_data_module (resource_data_id);

-- jig_data -> resource_data
with cte as (
    select (array_agg(jd.id))[1] as id
    from jig_data "jd"
          inner join jig on (draft_id = jd.id or live_id = jd.id)
          left join jig_admin_data "admin" on admin.jig_id = jig.id
          left join jig_data_additional_resource "resource" on jd.id = resource.jig_data_id
    where jig_focus = 1
    group by updated_at, created_at, jig.published_at, admin.jig_id
)
insert into resource_data(id, draft_or_live, display_name, language, description, privacy_level,
                          other_keywords, translated_keywords, translated_name, translated_description,
                          locked, created_at, updated_at)
select cte.id,
       draft_or_live,
       display_name,
       language,
       description,
       privacy_level,
       other_keywords,
       translated_keywords,
       translated_name,
       translated_description,
       locked,
       created_at,
       updated_at
from cte
inner join jig_data on cte.id = jig_data.id;


-- jig -> resource
insert into resource(id, creator_id, author_id, parents, live_id, draft_id, published_at, likes, plays)
select jig.id,
       creator_id,
       author_id,
       parents,
       live_id,
       draft_id,
       published_at,
       liked_count,
       (select count(*) from jig_play_count where jig.id = jig_play_count.jig_id)
from jig
where jig_focus = 1;


-- jig_data_module -> resource_data_module
insert into resource_data_module(id, index, resource_data_id, kind, is_complete, contents, created_at, updated_at)
select  jdm.id,
        index,
        jig_data_id,
        kind,
        is_complete,
        contents,
        jdm.created_at,
        jdm.updated_at
from jig_data_module jdm
inner join resource_data rd on rd.id = jdm.jig_data_id;


-- jig_data_additional_resource -> resource_data_resource
insert into resource_data_resource(
    id,
    resource_data_id,
    display_name,
    resource_type_id,
    resource_content)
select jdar.id,
       jig_data_id,
       jdar.display_name,
       jdar.resource_type_id,
       jdar.resource_content
from jig_data_additional_resource jdar
inner join resource_data rd on rd.id = jdar.jig_data_id;


-- jig_data_affiliation -> resource_data_affiliation
insert into resource_data_affiliation(resource_data_id, affiliation_id)
select jig_data_id, affiliation_id
from jig_data_affiliation jda
inner join resource_data rd on rd.id = jda.jig_data_id;


-- jig_data_age_range -> resource_data_age_range
insert into resource_data_age_range(resource_data_id, age_range_id)
select jig_data_id,
       age_range_id
from jig_data_age_range jdar
inner join resource_data rd on rd.id = jdar.jig_data_id;


-- jig_data_category -> resource_data_category
insert into resource_data_category(resource_data_id, category_id)
select jig_data_id,
       category_id
from jig_data_category jdar
inner join resource_data rd on rd.id = jdar.jig_data_id;


-- jig_like -> resource_like
insert into resource_like(resource_id, user_id, created_at)
select jig_id,
       user_id,
       jl.created_at
from jig_like jl
inner join "resource" r on r.id = jl.jig_id;

-- jig_admin_data -> resource_admin_data
insert into resource_admin_data(rating, blocked, curated, resource_id)
select rating,
       blocked,
       curated,
       jig_id
from jig_admin_data jad
inner join "resource" r on r.id = jad.jig_id;

-- jig_curation_comment -> resource_curation_comment
insert into resource_curation_comment(id, resource_id, comment, author_id, created_at)
select jcc.id,
       jig_id,
       comment,
       jcc.author_id,
       jcc.created_at
from jig_curation_comment jcc
inner join "resource" r on r.id = jcc.jig_id;

-- jig_report -> resource_report
insert into resource_report(id, resource_id, report_type, reporter_id, created_at
)
select jr.id, 
       jig_id, 
       report_type, 
       reporter_id, 
       jr.created_at
from jig_report jr
inner join "resource" r on r.id = jr.jig_id;

-- jig_curation_data -> resource_curation_data
insert into resource_curation_data(resource_id, display_name, language, categories , description, age_ranges, affiliations, additional_resources, curation_status, updated_at)
select  jig_id,
        display_name,
        language,
        categories ,
        description,
        age_ranges,
        affiliations,
        additional_resources,
        curation_status,
        jcd.updated_at
from jig_curation_data jcd
inner join "resource" r on r.id = jcd.jig_id;
