alter table jig_module rename to jig_module_tmp;

create table jig_module (
    id         uuid primary key not null default uuid_generate_v1mc(),
    "index"    int2             not null check ("index" >= 0),
    jig_id     uuid             not null references jig(id) on delete cascade,
    kind       int2,
    contents   jsonb,
    created_at timestamptz      not null default now(),
    updated_at timestamptz,
    unique("index", jig_id) deferrable initially deferred,
    check("index" <> 0 or kind is not distinct from 0)
);

insert into jig_module (id, "index", jig_id, kind, contents, created_at, updated_at)
select id, "index", jig_id, kind, contents, created_at, updated_at
from jig_module_tmp
inner join module on jig_module_tmp.module_id = module.id;

drop table jig_module_tmp;
drop table module;

-- "ManageModule" doesn't exist anymore.
delete from user_scope where scope = 5;
