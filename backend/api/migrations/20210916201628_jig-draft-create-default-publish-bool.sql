-- Add migration script here
alter table jig
    add column if not exists is_draft      bool     not null default false,
    add column if not exists privacy_level smallint not null default 1;
-- 1 is unlisted

update jig
set is_draft = true
where publish_at is distinct from null
  and publish_at < now();

update jig
set privacy_level = 0
where is_public is true;

alter table jig
    drop column if exists is_public;

-- make all existing jigs into super-jigs with the same IDs
-- copy any data they have into the draft and live "jig_metadata" tables to store the data
-- update tables FK'd to jig to reference the jig_metadata table instead?
-- maybe make new tables and insert?
-- modules?


create table jig_metadata
(
    id                      uuid        default uuid_generate_v1mc() primary key,
    jig_id                  uuid                                 not null references jig (id),
    display_name            text        default ''::text         not null,
    creator_id              uuid
        constraint jig_creator_id_fkey
            references "user"
            on delete set null,
    author_id               uuid
        constraint jig_author_id_fkey
            references "user"
            on delete set null,
    created_at              timestamptz default now()            not null,
    updated_at              timestamptz,
    parents                 uuid[]      default '{}'::uuid[]     not null,
    language                text                                 not null,
    last_synced_at          timestamptz,
    description             text        default ''::text         not null,
    theme                   smallint    default 0                not null,
    audio_background        smallint,
    audio_feedback_negative smallint[]  default '{}'::smallint[] not null,
    audio_feedback_positive smallint[]  default '{}'::smallint[] not null,
    direction               smallint    default 0                not null,
    display_score           boolean     default false            not null,
    drag_assist             boolean     default false            not null,
    track_assessments       boolean     default false            not null,
    privacy_level           smallint    default 1                not null
);

alter table jig
    add column live_id  uuid references jig_metadata (id),
    add column draft_id uuid references jig_metadata (id),
    add constraint jig_check check (live_id != draft_id);


alter table jig
    rename column publish_at to published_at;

alter table jig_module
    rename to jig_module_tmp;
alter table jig_affiliation
    rename to jig_affiliation_tmp;
alter table jig_category
    rename to jig_category_tmp;
alter table jig_goal
    rename to jig_goal_tmp;
alter table jig_age_range
    rename to jig_age_range_tmp;
alter table jig_additional_resource
    rename to jig_additional_resource_tmp;

create table jig_module (
    id          uuid                     default uuid_generate_v1mc() not null
        constraint jig_module_pkey
            primary key,
    index       smallint                                              not null
        constraint jig_module_index_check1
            check (index >= 0),
    jig_id      uuid                                                  not null
        constraint jig_module_jig_id_fkey1
            references jig_metadata
            on delete cascade,
    kind        smallint                                              not null,
    contents    jsonb                    default '{}'::jsonb          not null,
    created_at  timestamp with time zone default now()                not null,
    updated_at  timestamp with time zone default now()                not null,
    is_complete boolean                  default false                not null,
    constraint jig_module_index_jig_id_key1
        unique (index, jig_id)
            deferrable initially deferred,
    constraint jig_module_check
        check ((index <> 0) OR (NOT (kind IS DISTINCT FROM 0)))
);


create or replace function insert_into_jig_metadata(old_jig_id uuid[]) returns void
    language plpgsql
as
$$
declare
    id uuid;
begin
    foreach id in array old_jig_id
        loop
            select insert_into_jig_metadata_inner(id, true);
            select insert_into_jig_metadata_inner(id, false);
        end loop;
end;
$$;

create or replace function insert_into_jig_metadata_inner(old_jig_id uuid, is_live bool) returns void
    language plpgsql
as
$$
declare
    new_jig_metadata_id uuid;
begin
    insert
    into jig_metadata (jig_id, display_name, creator_id, author_id, created_at, updated_at, parents, language,
                       last_synced_at, description, privacy_level, direction, display_score, theme,
                       audio_background, audio_feedback_negative, audio_feedback_positive, track_assessments,
                       drag_assist)
        (select id,
                display_name,
                creator_id,
                author_id,
                created_at,
                updated_at,
                parents,
                language,
                last_synced_at,
                description,
                privacy_level,
                direction,
                display_score,
                theme,
                audio_background,
                audio_feedback_negative,
                audio_feedback_positive,
                track_assessments,
                drag_assist
         from jig
         where id = old_jig_id::uuid)
    returning id into new_jig_metadata_id;

    if is_live then
        update jig set live_id = new_jig_metadata_id where id = old_jig_id;
    else
        update jig set draft_id = new_jig_metadata_id where id = old_jig_id;
    end if;

    insert into jig_module ("index", jig_id, kind, contents)
    select "index", new_jig_metadata_id as "jig_id", kind, contents
    from jig_module
    where jig_id = old_jig_id;

    insert into jig_affiliation(jig_id, affiliation_id)
    select new_jig_metadata_id, affiliation_id
    from jig_affiliation
    where jig_id = old_jig_id;

    insert into jig_category(jig_id, category_id)
    select new_jig_metadata_id, category_id
    from jig_category
    where jig_id = old_jig_id;

    insert into jig_goal(jig_id, goal_id)
    select new_jig_metadata_id, goal_id
    from jig_goal
    where jig_id = old_jig_id;

    insert into jig_age_range(jig_id, age_range_id)
    select new_jig_metadata_id, age_range_id
    from jig_age_range
    where jig_id = old_jig_id;

    insert into jig_additional_resource(jig_id, url)
    select new_jig_metadata_id, url
    from jig_additional_resource
    where jig_id = old_jig_id;
end;
$$;


alter table jig
    drop column display_name,
    drop column creator_id,
    drop column author_id,
    drop column updated_at,
    drop column parents,
    drop column language,
    drop column last_synced_at,
    drop column description,
    drop column direction,
    drop column display_score,
    drop column theme,
    drop column audio_background,
    drop column audio_feedback_negative,
    drop column audio_feedback_positive,
    drop column track_assessments,
    drop column drag_assist,
    drop column is_draft,
    drop column privacy_level;


alter table jig
    alter column live_id set not null,
    alter column draft_id set not null;

drop function insert_into_jig_metadata_inner(uuid[]);
