alter table session
    add column last_used timestamptz default null;
