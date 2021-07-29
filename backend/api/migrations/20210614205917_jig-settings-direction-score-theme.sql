-- Add migration script here
alter table jig
    add column direction smallint not null default 0,
    add column display_score bool not null default false,
    add column theme smallint not null default 0;

