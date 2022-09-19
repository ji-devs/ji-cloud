alter table jig
    add column live_up_to_date bool default false not null;

alter table course
    add column live_up_to_date bool default false not null;

alter table resource
    add column live_up_to_date bool default false not null;

-- change flags to true if jig draft and live are current
with cte as (
    select jig.id as jig_id
    from jig_data
    inner join jig on jig.draft_id = jig_data.id
    where published_at is not null and published_at::timestamptz > updated_at::timestamptz
)
update jig
set live_up_to_date = true
from (select * from cte) subquery
where jig.id = subquery.jig_id;

-- change flags to true if course draft and live are current
with cte as (
    select course.id as course_id
    from course_data
    inner join course on course.draft_id = course_data.id
    where published_at is not null and published_at::timestamptz > updated_at::timestamptz
)
update course
set live_up_to_date = true
from (select * from cte) subquery
where course.id = subquery.course_id;

-- change flags to true if resource draft and live are current
with cte as (
    select resource.id as resource_id
    from resource_data
    inner join resource on resource.draft_id = resource_data.id
    where published_at is not null and published_at::timestamptz > updated_at::timestamptz
)
update resource
set live_up_to_date = true
from (select * from cte) subquery
where resource.id = subquery.resource_id;


-- updates live_up_to_date flag to false if a recent change was made to jig draft
create function jig_update_live_up_to_date() returns trigger
    language plpgsql
    as
$$
    declare
        published_at timestamptz := (select published_at from jig where jig.draft_id = new.id or jig.live_id = new.id);
    begin
        update jig
        set live_up_to_date = false
        where jig.draft_id = new.id;
        return null;
    END;
$$;

create trigger jig_update_on_draft
    after update of updated_at
    on jig_data
    for each row
execute procedure jig_update_live_up_to_date();

-- updates live_up_to_date flag to false if live is current to draft for jig
create function jig_publish_live_up_to_date() returns trigger
    language plpgsql
    as
$$
    begin
        update jig
        set live_up_to_date = true
        where jig.id = new.id and old.live_up_to_date = false;
        return null;
    END;
$$;

create trigger jig_update_on_publish
    after update of published_at
    on jig
    for each row
execute procedure jig_publish_live_up_to_date();


-- updates live_up_to_date flag to false if a recent change was made to course draft
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
    after update of updated_at
    on course_data
    for each row
execute procedure course_update_live_up_to_date();

-- updates live_up_to_date flag to false if live is current to draft for course
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
    after update of published_at
    on course
    for each row
execute procedure course_publish_live_up_to_date();

-- updates live_up_to_date flag to false if a recent change was made to resource draft
create function resource_update_live_up_to_date() returns trigger
    language plpgsql
    as
$$
    declare
        published_at timestamptz := (select published_at from resource where resource.draft_id = new.id or resource.live_id = new.id);
    begin
        update resource
        set live_up_to_date = false
        where resource.draft_id = new.id;
        return null;
    END;
$$;

create trigger resource_update_on_draft
    after update of updated_at
    on resource_data
    for each row
execute procedure resource_update_live_up_to_date();

-- updates live_up_to_date flag to false if live is current to draft for resource
create function resource_publish_live_up_to_date() returns trigger
    language plpgsql
    as
$$
    begin
        update resource
        set live_up_to_date = true
        where resource.id = new.id and old.live_up_to_date = false;
        return null;
    END;
$$;

create trigger resource_update_on_publish
    after update of published_at
    on resource
    for each row
execute procedure resource_publish_live_up_to_date();
