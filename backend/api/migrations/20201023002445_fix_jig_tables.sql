create table module (
    id uuid not null primary key,
    created_at   timestamptz   not null default now(),
    updated_at   timestamptz,
    kind         int2,
    contents jsonb
);

alter table jig alter column display_name drop not null;
alter table jig drop column cover;
alter table jig drop column ending;
alter table jig add column cover_id uuid references module(id) not null;
alter table jig add column ending_id uuid references module(id) not null;


alter table jig_module drop column module;
alter table jig_module add column module_id uuid references module(id) on delete cascade;
alter table jig_module add column "index"   int2 not null check("index" >= 0);
alter table jig_module add unique("index", jig_id) deferrable initially deferred;
