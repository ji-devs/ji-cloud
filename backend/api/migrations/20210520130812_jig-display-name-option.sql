update jig
set display_name = ''
where display_name is null;

alter table jig
    alter column display_name set not null,
    alter column display_name set default '';
