-- flag to prevent repeated language detection for Jig
alter table jig
    add column name_translate_status smallint,
    add column description_translate_status smallint;

update jig
    set name_translate_status = (case
                                        when jd.display_name <> '' and jd.translated_name <> '{}' then 2
                                        else name_translate_status
                                   end),
         description_translate_status = (case
                                        when description <> '' and translated_description <> '{}' then 2
                                        else description_translate_status
                                   end)
from (select jd.id      as "jig_data_id",
           display_name,
           translated_name,
           description,
           translated_description
    from jig_data jd
    inner join jig on jd.id = jig.live_id
    where published_at is not null) as jd
where jig.live_id = jd.jig_data_id;

-- flag to prevent repeated language detection for Image
alter table image_metadata
    add column name_translate_status smallint,
    add column description_translate_status smallint;

update image_metadata im
    set name_translate_status = (case
                                        when name <> '' and translated_name <> '{}' then 2
                                        else name_translate_status
                                   end),
        description_translate_status = (case
                                        when description <> '' and translated_description <> '{}' then 2
                                        else description_translate_status
                                   end)
from (select image_id, processed_at from image_upload where processed_at is not null) as iu
where im.id = iu.image_id;

-- flag to prevent repeated language detection for Course
alter table course
    add column name_translate_status smallint,
    add column description_translate_status smallint;

update course
    set name_translate_status = (case
                                        when cd.display_name <> '' and cd.translated_name <> '{}' then 2
                                        else name_translate_status
                                   end),
         description_translate_status = (case
                                        when description <> '' and translated_description <> '{}' then 2
                                        else description_translate_status
                                   end)
from (select cd.id      as "course_data_id",
           display_name,
           translated_name,
           description,
           translated_description
    from course_data cd
    inner join course on cd.id = course.live_id
    where published_at is not null) as cd
where course.live_id = cd.course_data_id;

-- flag to prevent repeated language detection for Resource
alter table resource
    add column name_translate_status smallint,
    add column description_translate_status smallint;

update resource
    set name_translate_status = (case
                                        when rd.display_name <> '' and rd.translated_name <> '{}' then 2
                                        else name_translate_status
                                   end),
         description_translate_status = (case
                                        when description <> '' and translated_description <> '{}' then 2
                                        else description_translate_status
                                   end)
from (select rd.id      as "resource_data_id",
           display_name,
           translated_name,
           description,
           translated_description
    from resource_data rd
    inner join resource on rd.id = resource.live_id
    where published_at is not null) as rd
where resource.live_id = rd.resource_data_id;

--
-- Null Status for Course
--
create function course_translate_des_status() returns trigger
    language plpgsql
as
$$
begin
    update course
    set description_translate_status = null
    where live_id = new.id;
    return NEW;
end;
$$;

create trigger course_translate_des_status
    after update of description
    on course_data
    for each row
execute procedure course_translate_des_status();

create function course_translate_name_status() returns trigger
    language plpgsql
as
$$
begin
    update course
    set name_translate_status = null
    where live_id = new.id;
    return NEW;
end;
$$;

create trigger course_translate_name_status
    after update of display_name
    on course_data
    for each row
execute procedure course_translate_name_status();

--
-- Null Status for Jig
--
create function jig_translate_des_status() returns trigger
    language plpgsql
as
$$
begin
    update jig
    set description_translate_status = null
    where live_id = new.id;
    return NEW;
end;
$$;

create trigger jig_translate_des_status
    after update of description
    on jig_data
    for each row
execute procedure jig_translate_des_status();

create function jig_translate_name_status() returns trigger
    language plpgsql
as
$$
begin
    update jig
    set name_translate_status = null
    where live_id = new.id;
    return NEW;
end;
$$;

create trigger jig_translate_name_status
    after update of display_name
    on jig_data
    for each row
execute procedure jig_translate_name_status();

--
-- Null Status for Resource
--
create function resource_translate_des_status() returns trigger
    language plpgsql
as
$$
begin
    update resource
    set description_translate_status = null
    where live_id = new.id;
    return NEW;
end;
$$;

create trigger resource_translate_des_status
    after update of description
    on resource_data
    for each row
execute procedure resource_translate_des_status();

create function resource_translate_name_status() returns trigger
    language plpgsql
as
$$
begin
    update resource
    set name_translate_status = null
    where live_id = new.id;
    return NEW;
end;
$$;

create trigger resource_translate_name_status
    after update of display_name
    on resource_data
    for each row
execute procedure resource_translate_name_status();

--
-- Null Status for Image
--
create function image_translate_des_status() returns trigger
    language plpgsql
as
$$
begin
    update image_metadata
    set description_translate_status = null
    where id = new.id;
    return NEW;
end;
$$;

create trigger image_translate_des_status
    after update of description
    on image_metadata
    for each row
execute procedure image_translate_des_status();

create function image_translate_name_status() returns trigger
    language plpgsql
as
$$
begin
    update image_metadata
    set name_translate_status = null
    where id = new.id;
    return NEW;
end;
$$;

create trigger image_translate_name_status
    after update of name
    on image_metadata
    for each row
execute procedure image_translate_name_status();
