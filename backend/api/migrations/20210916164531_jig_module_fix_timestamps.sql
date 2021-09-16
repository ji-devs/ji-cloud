alter table jig_module alter column updated_at set default now(),
    alter column updated_at set not null;
