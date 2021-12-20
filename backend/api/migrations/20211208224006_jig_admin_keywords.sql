alter table jig_data 
    add column locked boolean           not null default false,
    add other_keywords text             not null default '',
    add translated_keywords text        not null default '';

create table jig_admin_data (
    jig_data_id      uuid primary key references jig_data (id) on delete cascade,

    rating           int2,                                  -- jig rating from Admin, priortize higher rated jigs

    blocked          boolean not null default false,        -- true if jig is cloked from search results

    curated          boolean not null default false         -- true if admin has curated jig
)
