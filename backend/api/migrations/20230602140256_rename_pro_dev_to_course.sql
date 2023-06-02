-- drop pre existing indices
drop index pro_dev_draft_id;
drop index pro_dev_live_id;
drop index pro_dev_data_category_pro_dev_data_id_idx;
drop index pro_dev_data_resource_pro_dev_data_id_idx;
drop index pro_dev_data_module_pro_dev_data_id_idx;
drop index pro_dev_data_unit_pro_dev_data_id_idx;
drop index pro_dev_like_pro_dev_id_idx;


-- drop pre existing triggers
drop trigger pro_dev_update_on_draft on pro_dev_data;
drop trigger pro_dev_update_on_publish on pro_dev;
drop trigger pro_dev_like on pro_dev_like;
drop trigger pro_dev_unlike on pro_dev_like;

-- drop pre existing functions
drop function pro_dev_update_live_up_to_date;
drop function pro_dev_publish_live_up_to_date;
drop function add_pro_dev_like;
drop function sub_pro_dev_like;




-- rename tables
alter table pro_dev_data_module RENAME to course_data_module;
alter table pro_dev_data RENAME to course_data;
alter table pro_dev RENAME to course;
alter table pro_dev_data_category RENAME to course_data_category;
alter table pro_dev_data_resource RENAME to course_data_resource;
alter table pro_dev_like RENAME to course_like;
alter table pro_dev_data_unit RENAME to course_data_unit;




-- rename columns
alter table course_like RENAME COLUMN pro_dev_id to course_id;
alter table course_data_unit RENAME COLUMN pro_dev_data_id to course_data_id;
alter table course_data_module RENAME COLUMN pro_dev_data_id to course_data_id;
alter table course_data_category RENAME COLUMN pro_dev_data_id to course_data_id;
alter table course_data_resource RENAME COLUMN pro_dev_data_id to course_data_id;




-- -- rename constraint ==========================================================================================================================================================================
-- alter table playlist_data_resource drop constraint course_data_resource_course_data_id_fkey;
-- alter table playlist_data_resource add constraint playlist_data_resource_playlist_data_id_fkey foreign key (playlist_data_id) references playlist_data(id) on delete cascade;

-- alter table playlist_data_resource drop constraint course_data_resource_resource_type_id_fkey;
-- alter table playlist_data_resource add constraint playlist_data_resource_resource_type_id_fkey foreign key (resource_type_id) references resource_type(id) on delete cascade;

-- alter table playlist_data_jig drop constraint course_data_jig_course_data_id_jig_id_index_key;
-- alter table playlist_data_jig add constraint playlist_data_jig_playlist_data_id_jig_id_index_key unique(playlist_data_id, jig_id, index);




-- rename indexes
create index course_draft_id
    on course (draft_id);
create index course_live_id
    on course (live_id);
create index course_data_category_course_data_id_idx
    on course_data_category (course_data_id);
create index course_data_resource_course_data_id_idx
    on course_data_resource (course_data_id);
create index course_data_module_course_data_id_idx
    on course_data_module (course_data_id);
create index course_data_unit_course_data_id_idx
    on course_data_unit (course_data_id);
create index course_like_course_id_idx
    on course_like (course_id);



-- recreate functions and trigger for playlists
create function course_update_live_up_to_date() returns trigger
    language plpgsql
as
$$
    declare
        published_at timestamptz := (select published_at from course where course.draft_id = new.id or course.live_id = new.id);
    begin
        update course
        set live_up_to_date = false
        where course.draft_id = new.id;
        return null;
        
    END;
$$;

create trigger course_update_on_draft
    after update
        of updated_at
    on course_data
    for each row
execute procedure course_update_live_up_to_date();


create function course_publish_live_up_to_date() returns trigger
    language plpgsql
as
$$
    begin
        update course
        set live_up_to_date = true
        where course.id = new.id and old.live_up_to_date = false;
        return null;
    END;
$$;

create trigger course_update_on_publish
    after update
        of published_at
    on course
    for each row
execute procedure course_publish_live_up_to_date();


create function add_course_like() returns trigger
    language plpgsql
as
$$
begin
    update course
    set likes = likes + 1
    where id = NEW.course_id;
    return NEW;
end;
$$;

create trigger course_like
    after insert
    on course_like
    for each row
execute procedure add_course_like();


create function sub_course_like() returns trigger
    language plpgsql
as
$$
begin
    update course
    set likes = likes - 1
    where id = OLD.course_id;
    return NULL;
end;
$$;

create trigger course_unlike
    after delete
    on course_like
    for each row
execute procedure sub_course_like();


create trigger bump_course_updated
    after update
    on course_data
    for each row
    when (old.* IS DISTINCT FROM new.*)
execute procedure set_updated_at();
