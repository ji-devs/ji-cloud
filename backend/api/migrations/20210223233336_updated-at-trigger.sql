create function set_updated_at() returns trigger
    language plpgsql
as
$$
begin
    new.updated_at = now();
    return new;
end;
$$;

create function trigger_updated_at(tbl regclass) returns void
    language plpgsql
as
$$
begin
    -- https://stackoverflow.com/a/10711349
    execute format('create trigger set_updated_at
        before update
        on %s
        for each row
        when (old is distinct from new)
    execute function set_updated_at();', tbl);
end;
$$;


select trigger_updated_at('locale_entry');
