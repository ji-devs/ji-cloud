alter table jig_module
    alter column updated_at set default now();

update jig_module
set updated_at = now()
where updated_at is not distinct from null;

alter table jig_module
    alter column updated_at set not null;
