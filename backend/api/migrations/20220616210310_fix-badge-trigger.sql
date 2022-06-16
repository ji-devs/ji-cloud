-- Fixes
-- 1. incorrect variable name for badge member functions and 
-- 2. rename function and trigger names

drop trigger add_members_count on badge_member;
drop trigger members_count_leave on badge_member;
drop function update_members_count_join();
drop function update_members_count_leave();
drop function bump_badge_updated_at(); 

alter table badge_member
    add column joined_at        timestamp with time zone      not null    default now();

create function update_member_count_join() returns trigger
    language plpgsql
as
$$
begin
    update badge
    set member_count = member_count + 1
    where id = NEW.id;
    return NEW;
end;
$$;

create trigger member_count_add
    after insert
    on badge_member
    for each row
execute procedure update_member_count_join();

create function update_member_count_leave() returns trigger
    language plpgsql
as
$$
begin
    update badge
    set member_count = member_count - 1
    where id = OLD.id;
    return NULL;
end;
$$;

create trigger member_count_leave
    after delete
    on badge_member
    for each row
execute procedure update_member_count_leave();

create function bump_badge_updated_at() returns trigger
    language plpgsql
as
$$
begin
    update badge set last_synced_at = null where new.id = badge.id or old.id = badge.id;
    return null;
end;
$$;

create trigger bump_badge_updated
    after insert or delete
    on badge_member
    for each row
execute procedure bump_badge_updated_at();

