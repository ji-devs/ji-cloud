alter table jig_admin_data 
    drop column jig_data_id;

alter table jig_admin_data
    add column jig_id uuid primary key references jig (id) on delete cascade;

insert into jig_admin_data(jig_id)
select id 
from jig;

create or replace function update_jig_admin()
    returns trigger as
$$
begin
    insert into jig_admin_data(jig_id)
    select id
    from jig
    where id = NEW.id;
    return NEW;
end;
$$
    language plpgsql;

create trigger add_jig_admin
    after insert
    on jig
    for each row
execute function update_jig_admin();