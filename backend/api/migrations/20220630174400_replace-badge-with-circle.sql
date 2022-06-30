drop trigger member_count_add on badge_member;
drop trigger member_count_leave on badge_member;
drop function update_member_count_join() cascade;
drop function update_member_count_leave() cascade;
drop function bump_badge_updated_at() cascade;

drop table badge cascade;
drop table badge_member cascade;

create table circle
(
    id                      uuid                     default uuid_generate_v1mc() not null
        primary key,
    display_name            text                     default ''::text             not null,
    description             text                     default ''::text             not null,
    creator_id               uuid                                                 not null
            references "user" (id)           on delete cascade,
    member_count           bigint                   default 0                    not null,
            constraint circle_member_count_check
            check (member_count >= 0),
    image              uuid
            references "image_metadata" (id)
            on delete cascade,
    created_at              timestamp with time zone default now()                not null,
    updated_at              timestamp with time zone,
    last_synced_at timestamp with time zone

);

create table circle_member
(
    id        uuid                                   not null
        references circle
            on delete cascade,
    user_id   uuid                                   not null
        references "user"
            on delete cascade,
    joined_at timestamp with time zone default now() not null,
    unique (id, user_id)
);

create function update_member_count_join() returns trigger
    language plpgsql
as
$$
begin
    update circle
    set member_count = member_count + 1
    where id = NEW.id;
    return NEW;
end;
$$;

create trigger member_count_add
    after insert
    on circle_member
    for each row
execute procedure update_member_count_join();

create function update_member_count_leave() returns trigger
    language plpgsql
as
$$
begin
    update circle
    set member_count = member_count - 1
    where id = OLD.id;
    return NULL;
end;
$$;

create trigger member_count_leave
    after delete
    on circle_member
    for each row
execute procedure update_member_count_leave();

create function bump_circle_updated_at() returns trigger
    language plpgsql
as
$$
begin
    update circle set updated_at = now() where new.id = circle.id or old.id = circle.id;
    return null;
end;
$$;

create trigger bump_circle_updated
    after insert or delete
    on circle_member
    for each row
execute procedure bump_circle_updated_at();

delete from algolia_index_settings where index_name = 'badge_index';

insert into algolia_index_settings(index_name)
values('circle_index');

