-- Triggers for Pro Dev and Units

insert into algolia_index_settings(index_name)
values
('pro_dev_index');

--
-- update updated_at field when there's
--
create function pro_dev_update_live_up_to_date() returns trigger
    language plpgsql
as
$$
    declare
        published_at timestamptz := (select published_at from pro_dev where pro_dev.draft_id = new.id or pro_dev.live_id = new.id);
    begin
        update pro_dev
        set live_up_to_date = false
        where pro_dev.draft_id = new.id;
        return null;
        
    END;
$$;

create trigger pro_dev_update_on_draft
    after update
        of updated_at
    on pro_dev_data
    for each row
execute procedure pro_dev_update_live_up_to_date();


--
-- Update live_up_to_date flag
--
create function pro_dev_publish_live_up_to_date() returns trigger
    language plpgsql
as
$$
    begin
        update pro_dev
        set live_up_to_date = true
        where pro_dev.id = new.id and old.live_up_to_date = false;
        return null;
    END;
$$;

create trigger pro_dev_update_on_publish
    after update
        of published_at
    on pro_dev
    for each row
execute procedure pro_dev_publish_live_up_to_date();

--
-- increment like count on pro_dev after a (user_id, pro_dev_id) pair is added. 
--
create function add_pro_dev_like() returns trigger
    language plpgsql
as
$$
begin
    update pro_dev
    set likes = likes + 1
    where id = NEW.pro_dev_id;
    return NEW;
end;
$$;

create trigger pro_dev_like
    after insert
    on pro_dev_like
    for each row
execute procedure add_pro_dev_like();

--
-- decrement like count on pro_dev after a (user_id, pro_dev_id) pair is deleted. 
--
create function sub_pro_dev_like() returns trigger
    language plpgsql
as
$$
begin
    update pro_dev
    set likes = likes - 1
    where id = OLD.pro_dev_id;
    return NULL;
end;
$$;

create trigger pro_dev_unlike
    after delete
    on pro_dev_like
    for each row
execute procedure sub_pro_dev_like();

-- set updated_at field on update to metadata 
create trigger bump_pro_dev_updated
    after update
    on pro_dev_data
    for each row
    when (old.* IS DISTINCT FROM new.*)
execute procedure set_updated_at();



