create table jig_affiliation (
    jig_id uuid not null references jig (id) on delete cascade,
    affiliation_id uuid not null references affiliation (id)
);

create table jig_age_range (
    jig_id uuid not null references jig (id) on delete cascade,
    age_range_id uuid not null references age_range (id)
);

create function update_jig() returns trigger
    language plpgsql
as
$$
begin
    update jig set updated_at = now() where new.jig_id = jig.id or old.jig_id = jig.id;
    return null;
end;
$$;

alter table jig add column last_synced_at timestamptz;

create trigger bump_jig_updated after insert or delete on jig_category for each row execute procedure update_jig();
create trigger bump_jig_updated after insert or delete on jig_goal for each row execute procedure update_jig();
create trigger bump_jig_updated after insert or delete on jig_module for each row execute procedure update_jig();
