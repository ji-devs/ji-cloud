-- set no badge users

update user_profile
set badge = 10
where badge is null;

alter table user_profile
alter column badge set default 10,
alter column badge set not null;
