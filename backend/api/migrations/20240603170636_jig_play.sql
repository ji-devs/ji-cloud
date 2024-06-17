-- Jig Played Count 
alter table jig
    add column played_count int8 not null default 0 check ( played_count >= 0 );

create table jig_play
(
    jig_id     uuid              not null          references jig(id) on delete cascade,
    user_id    uuid              not null,
    at         timestamptz       not null          default now(),
    primary key (jig_id, user_id)
);

create or replace function update_jig_plays()
    returns trigger as
$$
begin
    update jig
    set played_count = played_count + 1
    where id = NEW.jig_id;
    return NEW;
end;
$$
    language plpgsql;

create trigger add_jig_plays
    after insert
    on jig_play
    for each row
execute function update_jig_plays();
