-- Add migration script here
alter table jig_player_session
    add column creator_id uuid references "user" (id) default null,
    add column name text default null;
