
insert into algolia_index_settings(index_name)
values
('resource_index');

--
-- function: update_resource_admin() 
-- trigger: add_resource_admin
-- description: add resource_id to resource_admin_data once added to resource
--
create or replace function update_resource_admin()
    returns trigger as
$$
begin
    insert into resource_admin_data(resource_id)
    select id
    from resource
    where id = NEW.id;
    return NEW;
end;
$$
    language plpgsql;

create trigger add_resource_admin
    after insert
    on resource
    for each row
execute procedure update_resource_admin();

--
-- function: resource_curation_add()
-- trigger: resource_curation_add
-- description: add resource_id to resource_curation_data once added to resource
--
create function resource_curation_add() returns trigger
    language plpgsql
as
$$
begin
    insert into resource_curation_data(resource_id)
    select id
    from resource
    where id = NEW.id;
    return NEW;
end;
$$;

create trigger resource_curation_add
    after insert
    on resource
    for each row
execute procedure resource_curation_add();

--
-- function: resource_unlike()
-- trigger: sub_resource_like
-- description: increment like count on resource after a (user_id, resource_id) pair is added. 
--
create function add_resource_like() returns trigger
    language plpgsql
as
$$
begin
    update resource
    set likes = likes + 1
    where id = NEW.resource_id;
    return NEW;
end;
$$;

create trigger resource_like
    after insert
    on resource_like
    for each row
execute procedure add_resource_like();

--
-- function: sub_resource_like()
-- trigger: resource_unlike
-- description: decrement like count on resource after a (user_id, resource_id) pair is deleted. 
--
create function sub_resource_like() returns trigger
    language plpgsql
as
$$
begin
    update resource
    set likes = likes - 1
    where id = OLD.resource_id;
    return NULL;
end;
$$;

create trigger resource_unlike
    after delete
    on resource_like
    for each row
execute procedure sub_resource_like();
