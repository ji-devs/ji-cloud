
-- drop pre existing learning path indices
DROP INDEX learning_path_draft_id;
DROP INDEX learning_path_live_id; 
DROP INDEX learning_path_data_resource_learning_path_data_id_idx; 
DROP INDEX learning_path_data_affiliation_learning_path_data_id_idx; 
DROP INDEX learning_path_data_age_range_learning_path_data_id_idx; 
DROP INDEX learning_path_data_category_learning_path_data_id_idx; 

-- drop pre existing learning path triggers
DROP TRIGGER add_learning_path_like on learning_path_like CASCADE;
DROP TRIGGER learning_path_unlike on learning_path_like CASCADE;
DROP TRIGGER bump_learning_path_data_updated_at on learning_path_data_age_range;
DROP TRIGGER bump_learning_path_data_updated_at on learning_path_data_affiliation;
DROP TRIGGER bump_learning_path_data_updated_at on learning_path_data_category;
DROP TRIGGER bump_learning_path_data_updated_at on learning_path_data_jig;

DROP FUNCTION update_learning_path_unlike();
DROP FUNCTION update_learning_path_like();

-- rename tables to course
ALTER TABLE learning_path_data_category
    RENAME TO course_data_category;
ALTER TABLE learning_path_data_age_range
    RENAME TO course_data_age_range;
ALTER TABLE learning_path_data_affiliation
    RENAME TO course_data_affiliation;
ALTER TABLE learning_path_data_resource
    RENAME TO course_data_resource;
ALTER TABLE learning_path_data_jig
    RENAME TO course_data_jig;
ALTER TABLE learning_path_like
    RENAME TO course_like;
ALTER TABLE learning_path
    RENAME TO course;
ALTER TABLE learning_path_data
    RENAME TO course_data;

-- rename to course_data_id
ALTER TABLE course_data_category RENAME COLUMN learning_path_data_id TO course_data_id;
ALTER TABLE course_data_age_range RENAME COLUMN learning_path_data_id TO course_data_id;
ALTER TABLE course_data_affiliation RENAME COLUMN learning_path_data_id TO course_data_id;
ALTER TABLE course_data_resource RENAME COLUMN learning_path_data_id TO course_data_id;
ALTER TABLE course_data_jig RENAME COLUMN learning_path_data_id TO course_data_id;
ALTER TABLE course_like RENAME COLUMN learning_path_id TO course_id;


-- rename indexes  
create index course_draft_id
    on course (draft_id);
create index course_live_id
    on course (live_id);
create index course_data_resource_course_data_id_idx
    on course_data_resource (course_data_id);
create index course_data_affiliation_course_data_id_idx
    on course_data_affiliation (course_data_id);
create index course_data_age_range_course_data_id_idx
    on course_data_age_range (course_data_id);
create index course_data_category_course_data_id_idx
    on course_data_category (course_data_id);
create index course_like_course_id_idx
    on course_like (course_id);

-- recreate functions and trigger for courses
create function update_course_like() returns trigger
    language plpgsql
as
$$
begin
    update course
    set liked_count = liked_count + 1
    where id = NEW.course_id;
    return NEW;
end;
$$;

create trigger add_course_like
    after insert
    on course_like
    for each row
execute procedure update_course_like();

create function update_course_unlike() returns trigger
    language plpgsql
as
$$
begin
    update course
    set liked_count = liked_count - 1
    where id = OLD.course_id;
    return NULL;
end;
$$;

create trigger course_unlike
    after delete
    on course_like
    for each row
execute procedure update_course_unlike();

create function bump_course_data_updated_at() returns trigger
    language plpgsql
as
$$
begin
    update course_data set updated_at = now() where new.course_data_id = course_data.id or old.course_data_id = course_data.id;
    return null;
end;
$$;

create trigger bump_course_data_updated_at after insert or delete on course_data_age_range for each row execute procedure bump_course_data_updated_at();
create trigger bump_course_data_updated_at after insert or delete on course_data_affiliation for each row execute procedure bump_course_data_updated_at();
create trigger bump_course_data_updated_at after insert or delete on course_data_category for each row execute procedure bump_course_data_updated_at();
create trigger bump_course_data_updated_at after insert or delete on course_data_jig for each row execute procedure bump_course_data_updated_at();

