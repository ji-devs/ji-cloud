create or replace function jig_unlike()
    returns trigger as
$$
begin
    update jig
    set liked_count = liked_count - 1
    where id = OLD.jig_id;
    return NULL;
end;
$$
    language plpgsql;

create trigger jig_unlike
    after delete
    on jig_like
    for each row
execute function jig_unlike();

