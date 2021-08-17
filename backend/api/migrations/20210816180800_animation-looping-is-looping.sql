-- Add migration script here
alter table animation_metadata
    rename column looping to is_looping;
