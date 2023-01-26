-- Separate public user last sunced at from user for Algolia
create table public_user
(
    user_id                     uuid                     not null
            references "user" (id)
            on delete cascade,

    last_synced_at              timestamp with time zone
);

insert into public_user(user_id, last_synced_at)
select user_id,
       last_synced_at
from user_profile;

update user_profile
set last_synced_at = null;
