create table badge
(
    id                      uuid                     default uuid_generate_v1mc() not null    
        primary key,
    display_name            text                     default ''::text             not null,
    description             text                     default ''::text             not null,
    creator_id               uuid                                                 not null
            references "user" (id)           on delete cascade,    
    member_count           bigint                   default 0                    not null,
            constraint badge_member_count_check
            check (member_count >= 0),
    thumbnail               text                     default ''::text             not null,
    created_at              timestamp with time zone default now()                not null,
    updated_at              timestamp with time zone
);

create table badge_member
(   
    id                      uuid                    not null
        references badge
            on delete cascade,
    user_id                 uuid                     not null
        references "user"(id)
            on delete cascade,
    
    unique(id, user_id)
);

create function update_members_count_join() returns trigger
    language plpgsql
as
$$
begin
    update badge
    set members_count = members_count + 1
    where id = NEW.id;
    return NEW;
end;
$$;

create trigger add_members_count
    after insert
    on badge_member
    for each row
execute procedure update_members_count_join();

create function update_members_count_leave() returns trigger
    language plpgsql
as
$$
begin
    update badge
    set members_count = members_count + 1
    where id = OLD.id;
    return NULL;
end;
$$;

create trigger members_count_leave
    after delete
    on badge_member
    for each row
execute procedure update_members_count_leave();

create function bump_badge_updated_at() returns trigger
    language plpgsql
as
$$
begin
    update badge set updated_at = now() where new.id = badge.id or old.id = badge.id;
    return null;
end;
$$;