create table user_asset_data (
    user_id                 uuid                                          not null
        primary key
        references "user"
            on delete cascade,
    jig_count               int      default 0                         not null     check(jig_count >= 0),
    playlist_count          int      default 0                         not null     check(playlist_count >= 0),
    course_count            int      default 0                         not null     check(course_count >= 0),
    resource_count          int      default 0                         not null     check(resource_count >= 0),
    total_asset_count       int      default 0                         not null     check(total_asset_count >= 0)
);

insert into user_asset_data(user_id)
select user_id
from user_profile;

create function add_user_to_asset_data() returns trigger
    language plpgsql
as
$$
begin
    insert into user_asset_data(user_id)
    select user_id
    from user_profile
    where user_id = NEW.user_id;
    return NEW;
end;
$$;

create trigger add_to_asset_data
    after insert
    on user_profile
    for each row
execute procedure add_user_to_asset_data();


create function sub_user_from_asset_data() returns trigger
    language plpgsql
as
$$
begin
    delete from user_asset_data
    where user_id = OLD.user_id;
    return null;
end;
$$;

create trigger sub_from_asset_data
    after delete
    on user_profile
    for each row
execute procedure sub_user_from_asset_data();

