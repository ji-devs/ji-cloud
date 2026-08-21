update user_profile
set updated_at = now()
where last_synced_at is not null;
