-- Add migration script here
alter table jig_player_session
alter column creator_id set not null;
