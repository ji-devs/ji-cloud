create table "settings" (
    singleton bool not null primary key default true check (singleton = true),
    algolia_index_version int2 not null default 0,
    algolia_index_name text not null
);
