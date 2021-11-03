alter table jig 
    add column initial_cover_set boolean not null default true;

alter table jig
    alter initial_cover_set set default false;
