alter table user_profile
    add column bio_public   bool    default false   not null;

update user_profile
set last_synced_at = null;
