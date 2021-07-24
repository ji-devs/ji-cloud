create table jig_code
(
    jig_id     uuid references "jig" (id) not null unique,
    index      smallint                   not null unique,
    created_at timestamptz                not null default now()
);

create function expired_jig_code() returns trigger
    language plpgsql
as
$$
begin
    delete from jig_code where created_at < now() - '2 weeks'::interval;
    return null;
end;
$$;

create trigger delete_expired_jig_code
    after insert on jig_code
    execute procedure expired_jig_code();
