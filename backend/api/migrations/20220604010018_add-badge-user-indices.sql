alter table user_profile
    add column last_synced_at  timestamp with time zone;

alter table badge
    add column last_synced_at  timestamp with time zone;


insert into algolia_index_settings(index_name)
values
('badge_index'), 
('public_user_index');


