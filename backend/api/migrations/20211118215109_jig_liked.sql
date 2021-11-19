-- Jig Liked Count 
alter table jig
    add column liked_count int8 not null default 0 check ( liked_count >= 0 );

create table jig_like
(
    jig_id     uuid              not null          references jig(id) on delete cascade,
    user_id    uuid              not null,
    created_at timestamptz       not null          default now(),
    primary key (jig_id, user_id)
);

create or replace function update_jig_likes()
    returns trigger as
$$
begin
    update jig
    set liked_count = liked_count + 1
    where id = NEW.jig_id;
    return NEW;
end;
$$
    language plpgsql;

create trigger add_jig_likes
    after insert
    on jig_like
    for each row
execute function update_jig_likes();



