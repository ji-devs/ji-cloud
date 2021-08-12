-- Add migration script here
create table jig_play_count
(
    jig_id uuid             not null references jig (id) on delete cascade,
    play_count   bigint        not null default 0
);
