--fix trigger "likes" variable
drop function if exists update_course_unlike cascade;
drop function if exists update_course_like cascade;

create function update_course_like() returns trigger
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
    set likes = likes - 1
    where id = OLD.course_id;
    return NULL;
end;
$$;

create trigger course_unlike
    after delete
    on course_like
    for each row
execute procedure update_course_unlike();
