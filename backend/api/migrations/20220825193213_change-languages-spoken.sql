alter table user_profile
    rename column language_spoken_public to languages_spoken_public;

alter table user_profile
      rename column language_spoken to languages_spoken;

update user_profile
set last_synced_at = null
where last_synced_at is not null;

