alter table course
    add column  parents  uuid[]   default '{}'::uuid[]         not null;

