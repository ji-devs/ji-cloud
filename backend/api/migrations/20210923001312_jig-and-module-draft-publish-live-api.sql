-- This migration reworks the schema of JIGs and all associated data to support draft copies.
-- (Sorry to anyone who has to read this loooong query)
--
--
-- # Why?
--
-- Previously, a JIG can either be a draft or published (live):
--  * messy notion of what is considered live
--      * some JIGs are associated with a draft copy through "jig_draft_join"
--          * move-copying changes in the draft copy to the live one publishes the draft's changes
--      * JIGs without an associated draft are published by setting its column "publish_at" < now()
--  * associated metadata (goals, affiliation, ...) and modules are FK'd to "jig" table
--
-- New design:
--  * JIGs have a draft and a live copy at all times
--      * "jig_data" can be either draft or live
--      * main "jig" table holds only FK references to "jig_data" table
--      * a jig is published by dropping all existing data and copying draft to live
--          * no user modifications to live "jig_data" rows directly
--  * associated metadata and modules are FK'd to "jig_data"
--  * modules have a "stable_id" in addition to "id"
--      * allows the modules to hold references to each other in their bodies ("contents" column) that do not break
--      when a draft jig is published to live
--
--
-- # What does this do?
--
--  0. !!! ASSUMES no existing drafts are in the database !!! (accurate as of 1632355662 unix time on both sandbox and release)
--  1. Preserves existing JIG ids and moves their columns into jig_data
--      * This is done twice: once for live, once for draft
--      * Info that differs between draft and live are copied s.t. they FK to the jig_data table rather than jig
--          * FK'd to jig_data: affiliation, age_ranges, module, category, goal, additional_resource
--          * FK'd to jig: play_count, player_session
--  2. Sets up all the triggers
--
--


create table jig_data
(
    id                      uuid        default uuid_generate_v1mc() primary key,
--     is_live                 bool                                 not null,
    display_name            text        default ''::text         not null,
    created_at              timestamptz default now()            not null,
    updated_at              timestamptz,
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
    track_assessments       boolean     default false            not null
);

alter table jig
    add column live_id      uuid references jig_data (id) on delete restrict,
    add column draft_id     uuid references jig_data (id) on delete restrict,
    add column published_at timestamptz,
    add constraint jig_check check (live_id <> draft_id);


create table jig_data_module
(
    id          uuid primary key     default uuid_generate_v1mc(),
    stable_id   uuid        not null default uuid_generate_v1mc(),
    "index"     int2        not null check ("index" >= 0),
    jig_data_id uuid        not null references jig_data (id) on delete cascade,
    kind        int2        not null,
    is_complete bool        not null default false,
    contents    jsonb       not null default '{}'::jsonb,
    created_at  timestamptz not null default now(),
    updated_at  timestamptz not null default now(),
    unique ("index", jig_data_id) deferrable initially deferred,
    check ("index" <> 0 or kind is not distinct from 0)
);

create table jig_data_affiliation
(
    jig_data_id    uuid not null references jig_data (id) on delete cascade,
    affiliation_id uuid not null references affiliation (id)
);

create table jig_data_category
(
    jig_data_id uuid not null references jig_data (id) on delete cascade,
    category_id uuid not null references category (id) on delete cascade
);

create table jig_data_goal
(
    jig_data_id uuid        not null references jig_data (id) on delete cascade,
    goal_id     uuid        not null references goal (id),
    created_at  timestamptz not null default now(),
    unique (jig_data_id, goal_id)
);

create table jig_data_age_range
(
    jig_data_id  uuid not null references jig_data (id) on delete cascade,
    age_range_id uuid not null references age_range (id)
);

create table jig_data_additional_resource
(
    id          uuid primary key default uuid_generate_v1mc(),
    jig_data_id uuid not null references jig_data (id) on delete cascade,
    url         text not null
);


create or replace function move_columns_jig_to_jig_data(old_jig_id uuid[]) returns void
    language plpgsql
as
$$
declare
    id       uuid;
    live_id  uuid;
    draft_id uuid;
begin
    foreach id in array old_jig_id
        loop
            raise notice 'jig.id: %', id;
            select move_columns_jig_to_jig_data_inner(id, true) into live_id;
            raise notice 'live_id: %', live_id;
            select move_columns_jig_to_jig_data_inner(id, false) into draft_id;
            raise notice 'draft_id: %', draft_id;
            perform insert_jig_data_module(id, live_id, draft_id);
        end loop;

    return;
end;
$$;

create or replace function move_columns_jig_to_jig_data_inner(old_jig_id uuid, is_live bool) returns uuid
    language plpgsql
as
$$
declare
    new_jig_data_id uuid;
begin
    insert
    into jig_data (display_name, created_at, updated_at, language, last_synced_at, description,
                   direction, display_score, theme, audio_background, audio_feedback_negative, audio_feedback_positive,
                   track_assessments, drag_assist)
        (select display_name,
                created_at,
                updated_at,
                language,
                last_synced_at,
                description,
                direction,
                display_score,
                theme,
                audio_background,
                audio_feedback_negative,
                audio_feedback_positive,
                track_assessments,
                drag_assist
         from jig
         where id = old_jig_id)
    returning id into new_jig_data_id;

    if is_live then
        update jig set live_id = new_jig_data_id where id = old_jig_id;
    else
        update jig set draft_id = new_jig_data_id where id = old_jig_id;
    end if;

    insert into jig_data_affiliation(jig_data_id, affiliation_id)
    select new_jig_data_id, affiliation_id
    from jig_affiliation
    where jig_id = old_jig_id;

    insert into jig_data_category(jig_data_id, category_id)
    select new_jig_data_id, category_id
    from jig_category
    where jig_id = old_jig_id;

    insert into jig_data_goal(jig_data_id, goal_id)
    select new_jig_data_id, goal_id
    from jig_goal
    where jig_id = old_jig_id;

    insert into jig_data_age_range(jig_data_id, age_range_id)
    select new_jig_data_id, age_range_id
    from jig_age_range
    where jig_id = old_jig_id;

    insert into jig_data_additional_resource(jig_data_id, url)
    select new_jig_data_id, url
    from jig_additional_resource
    where jig_id = old_jig_id;

    return new_jig_data_id;
end;
$$;

create or replace function insert_jig_data_module(old_jig_id uuid, live_id uuid, draft_id uuid) returns void
    language plpgsql
as
$$
declare
    stable_module_id    uuid;
    old_module_id_array uuid[];
    it                  uuid;
begin

    select array(select id from jig_module where jig_id = old_jig_id) into old_module_id_array;

    foreach it in array old_module_id_array
        loop
            insert into jig_data_module ("index", jig_data_id, kind, is_complete, contents, created_at,
                                         updated_at)
            select "index",
                   live_id as "jig_id",
                   kind,
                   is_complete,
                   contents,
                   created_at,
                   updated_at
            from jig_module
            where jig_module.id = it
            returning stable_id into stable_module_id;

            insert into jig_data_module (stable_id, "index", jig_data_id, kind, is_complete, contents, created_at,
                                         updated_at)
            select stable_module_id,
                   "index",
                   draft_id as "jig_id",
                   kind,
                   is_complete,
                   contents,
                   created_at,
                   updated_at
            from jig_module
            where jig_module.id = it;
        end loop;

    return;
end;
$$;

select move_columns_jig_to_jig_data(array(select id from jig));

alter table jig
    drop column display_name,
    drop column created_at,
    drop column updated_at,
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
    drop column publish_at;

alter table jig
    alter column live_id set not null,
    alter column draft_id set not null;

drop function move_columns_jig_to_jig_data_inner(uuid, bool);
drop function insert_jig_data_module(uuid, uuid, uuid);
drop function move_columns_jig_to_jig_data(uuid[]);

drop table jig_draft_join;
drop table jig_module;
drop table jig_affiliation;
drop table jig_category;
drop table jig_goal;
drop table jig_age_range;
drop table jig_additional_resource;

drop function update_jig();

create function bump_jig_data_updated_at() returns trigger
    language plpgsql
as
$$
begin
    update jig_data set updated_at = now() where new.jig_data_id = jig_data.id or old.jig_data_id = jig_data.id;
    return null;
end;
$$;


create trigger bump_jig_data_updated
    after insert or delete
    on jig_data_category
    for each row
execute procedure bump_jig_data_updated_at();
create trigger bump_jig_data_updated
    after insert or delete
    on jig_data_goal
    for each row
execute procedure bump_jig_data_updated_at();
create trigger bump_jig_data_updated
    after insert or delete or update
    on jig_data_module
    for each row
execute procedure bump_jig_data_updated_at();

drop trigger set_updated_at on jig;

select trigger_updated_at('jig_data');
select trigger_updated_at('jig_data_module');
