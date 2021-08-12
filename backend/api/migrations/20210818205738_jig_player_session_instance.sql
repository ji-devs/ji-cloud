create table jig_player_session_instance
(
    instance_id uuid        primary key     default uuid_generate_v1mc(),
    index smallint,
    jig_id uuid             not null references jig (id) on delete cascade,
    ip_addr text,
    user_agent text,
    constraint fk_jig_player_session
        foreign key (index)
            references jig_player_session(index) on delete cascade
);