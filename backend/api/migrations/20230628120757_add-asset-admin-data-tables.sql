-- Add admin data tables for courses and playlists;
-- Add is_premium flag to all asset types admin data tables.

create table if not exists course_admin_data
(
    rating  smallint,
    blocked boolean default false not null,
    curated boolean default false not null,
    is_premium boolean default false not null,
    course_id  uuid                  not null
        primary key
        references course (id)
            on delete cascade
)
;

insert into course_admin_data(course_id)
select id from course
;

create or replace function update_course_admin()
    returns trigger as
$$
begin
    insert into course_admin_data(course_id)
    select id
    from course
    where id = NEW.id;
    return NEW;
end;
$$
    language plpgsql;

create trigger add_course_admin
    after insert
    on course
    for each row
execute function update_course_admin();

create table if not exists playlist_admin_data
(
    rating  smallint,
    blocked boolean default false not null,
    curated boolean default false not null,
    is_premium boolean default false not null,
    playlist_id  uuid                  not null
        primary key
        references playlist (id)
            on delete cascade
)
;

insert into playlist_admin_data(playlist_id)
select id from playlist
;

create or replace function update_playlist_admin()
    returns trigger as
$$
begin
    insert into playlist_admin_data(playlist_id)
    select id
    from playlist
    where id = NEW.id;
    return NEW;
end;
$$
    language plpgsql;

create trigger add_playlist_admin
    after insert
    on playlist
    for each row
execute function update_playlist_admin();

alter table jig_admin_data
    add column if not exists is_premium boolean default false not null
;

alter table resource_admin_data
    add column if not exists is_premium boolean default false not null
;
