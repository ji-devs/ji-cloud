alter table jig
    add column description text default '',
    add column is_public bool not null default false;
