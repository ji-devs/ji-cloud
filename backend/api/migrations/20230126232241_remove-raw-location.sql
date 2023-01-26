-- Remove "raw" field from location in user_profile
update user_profile
set location = location->'raw',
    last_synced_at = null
where location->'raw' is not null;

