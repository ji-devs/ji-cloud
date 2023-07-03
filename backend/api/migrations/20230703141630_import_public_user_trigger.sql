insert into public_user(user_id)
select up.user_id
from user_profile "up"
left join public_user on public_user.user_id = up.user_id
where public_user.user_id is null;

create function add_user_to_public_user() returns trigger
    language plpgsql
as
$$
begin
    insert into public_user(user_id)
    select user_id
    from user_profile
    where user_id = NEW.user_id;
    return NEW;
end;
$$;

create trigger add_to_public_user
    after insert
    on user_profile
    for each row
execute procedure add_user_to_public_user();


create function sub_user_from_public_user() returns trigger
    language plpgsql
as
$$
begin
    delete from public_user
    where user_id = OLD.user_id;
    return null;
end;
$$;

create trigger sub_from_public_user
    after delete
    on user_profile
    for each row
execute procedure sub_user_from_public_user();
