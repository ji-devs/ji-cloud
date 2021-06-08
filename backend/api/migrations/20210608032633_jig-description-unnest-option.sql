update jig
    set description = ''
where description is not distinct from null;

alter table jig
alter column description set not null;
