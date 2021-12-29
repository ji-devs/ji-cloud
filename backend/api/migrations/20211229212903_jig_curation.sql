create table jig_curation_data (
    jig_id                      uuid          not null       references jig(id) on delete cascade,
    
    display_name                bool          not null   default false,
    language                    bool          not null   default false,
    goals                       bool          not null   default false,
    categories                  bool          not null   default false,    
    description                 bool          not null   default false, 
    age_ranges                  bool          not null   default false,    
    affiliations                bool          not null   default false, 
    additional_resources        bool          not null   default false,

    -- curation status of jig, default = NEW
    curation_status             int2          not null   default 0,

    updated_at                      timestamptz
);

create table jig_curation_comment (
    id          uuid              primary key    default uuid_generate_v1mc() not null,

    jig_id      uuid              not null       references jig(id) on delete cascade,

    comment       text            not null,

    author_id    uuid             not null       references "user"(id),

    created_at     timestamptz
);

insert into jig_curation_data(jig_id)
select id 
from jig;

create or replace function jig_curation_add()
    returns trigger as
$$
begin
    insert into jig_curation_data(jig_id)
    select id
    from jig
    where id = NEW.id;
    return NEW;
end;
$$
    language plpgsql;

create trigger jig_curation_add
    after insert
    on jig
    for each row
execute function jig_curation_add();

