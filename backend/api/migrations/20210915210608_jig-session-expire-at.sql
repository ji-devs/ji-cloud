alter table jig_player_session
    add column expires_at timestamptz not null default now() + interval '2 weeks';

drop trigger purge_expired_jig_player_session on jig_player_session;

create or replace function expired_jig_player_session() returns trigger
    language plpgsql
as
$$
begin
    delete from jig_player_session where expires_at < now();
    return null;
end;
$$;

create trigger purge_expired_jig_player_session
    before insert
    on jig_player_session
execute procedure expired_jig_player_session();

