-- Delete all existing dragdrop activities
delete from jig_data_module where kind=10;

-- Create a temporary table to hold the data. Using a CTE takes _really_ long
create table tmp_data_module
(
    id    uuid primary key,
    index smallint
)
;

-- Insert fixed indexes into the temp table
insert into tmp_data_module
select
  id,
  row_number () over (
    partition by jig_data_id
    order by index
  ) - 1 as index
  from jig_data_module
;

-- Update the jig_data_module table with the fixed indexes
update jig_data_module
  set index = subquery.index
from (select id, index from tmp_data_module) as subquery
where jig_data_module.id = subquery.id
;

-- Drop the temp table
drop table tmp_data_module;
