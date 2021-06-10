select 1 from jig_module where true for update;

update jig_module
    set contents = '{}'::jsonb
where contents is not distinct from null;

update jig_module
    set kind = 0
where contents is not distinct from null;

alter table jig_module
    alter column contents set not null,
    alter column contents set default '{}'::jsonb,
    alter column kind set not null;
