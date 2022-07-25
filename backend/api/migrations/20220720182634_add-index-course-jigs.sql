alter table course_data_jig 
    add column     index       smallint   check(index >= 0);

create table temp_course_jig (
    course_data_id      uuid,
    jig_id    uuid,
    index    smallint
);

-- Insert fixed indexes into the temp table
insert into temp_course_jig
select
  course_data_id,
  jig_id,
  row_number () over (
    partition by course_data_id
      order by index
  ) - 1 as index
  from course_data_jig;

delete from course_data_jig;

insert into course_data_jig(course_data_id, jig_id, index)
select * from temp_course_jig;

drop table if exists temp_course_jig;

create function process_jig_index() returns trigger
    language plpgsql
    as
$$
    declare
        count smallint := (select count(*) FROM course_data_jig
        where course_data_id = new.course_data_id);
    begin

        if (count = 0) then
            update course_data_jig
            set "index" = 0
            where course_data_id = new.course_data_id and jig_id = new.jig_id;
            return null; 
        else
            update course_data_jig
            set "index" = count - 1
            where course_data_id = new.course_data_id and jig_id = new.jig_id;
            return null; 
         end if;
    END;
$$;

create trigger jig_index
after insert on course_data_jig
    for each row 
execute procedure process_jig_index();
