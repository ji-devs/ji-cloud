create table jig_player_session_instance
(
    id            uuid primary key default uuid_generate_v1mc(),
    session_index smallint not null references jig_player_session (index) on delete cascade,
    ip_address    text,
    user_agent    text
);

create index on jig_player_session_instance (session_index);

alter table jig_player_session
    drop constraint jig_player_session_jig_id_key,
    drop constraint jig_player_session_index_check,
    add constraint jig_player_session_index_check check (index >= 0);
