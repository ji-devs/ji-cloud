create table jig_player_session
(
    index             smallint primary key check (index >= 0 and index < 10000),
    jig_id            uuid references "jig" (id) on delete cascade not null unique,
    created_at        timestamptz                                  not null default now(),
    direction         smallint                                     not null default 0,
    display_score     bool                                         not null default false,
    track_assessments bool                                         not null default false,
    drag_assist       bool                                         not null default false
);

create function expired_jig_player_session() returns trigger
    language plpgsql
as
$$
begin
    delete from jig_player_session where created_at < (now() - '2 weeks'::interval);
    return null;
end;
$$;

create trigger purge_expired_jig_player_session
    before insert
    on jig_player_session
execute procedure expired_jig_player_session();
