--
-- Fixes migrations/20220720182634_add-index-course-jigs.sql
--
-- Before: Recycle_metadata() method in API inserts multiple rows on a single transaction, causing the each new index
--         associated with course_data_id and jig_id to capture the last assigned index number.
--       
-- Now: When new rows are inserted within a single transaction, the trigger will take all null valued indexes within the transaction 
--      and index them after insert.

drop function if exists process_jig_index() cascade;

alter table course_data_jig drop constraint learning_path_data_jig_learning_path_data_id_jig_id_key;
alter table course_data_jig add constraint course_data_jig_course_data_id_jig_id_index_key unique(course_data_id, jig_id, index);

create table temp_course_jig (
    course_data_id      uuid,
    jig_id    uuid,
    index    smallint
);

insert into temp_course_jig
select
  course_data_id,
  jig_id,
  row_number () over (
    partition by course_data_id
      order by index
  ) - 1 as index
  from course_data_jig;

update course_data_jig
  set index = subquery.index
from (select course_data_id, jig_id, index from temp_course_jig) as subquery
where course_data_jig.course_data_id = subquery.course_data_id and course_data_jig.jig_id = subquery.jig_id;

drop table temp_course_jig;

create function process_jig_index() returns trigger
    language plpgsql
    as
$$
    begin
    create table temp_course_jig (
        course_data_id      uuid,
        jig_id    uuid,
        index    smallint
    );

    insert into temp_course_jig
    select
      course_data_id,
      jig_id,
      row_number () over (
        partition by course_data_id
          order by index
      ) - 1 as index
      from course_data_jig cdj
      where cdj.index is null;

    update course_data_jig
      set index = subquery.index
    from (select course_data_id, jig_id, index from temp_course_jig) as subquery
    where course_data_jig.course_data_id = subquery.course_data_id and course_data_jig.jig_id = subquery.jig_id;

    drop table temp_course_jig;

    return null;
    end;
$$;

-- "for each statement" instead of "for each row"
create trigger jig_index
after insert on course_data_jig
    for each statement
execute procedure process_jig_index();
