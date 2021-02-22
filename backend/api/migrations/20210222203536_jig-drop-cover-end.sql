update jig_module set index = index + 1;

insert into jig_module (jig_id, module_id, index)
select id, cover_id, 0
from jig
union all
select id, ending_id, (select max(index) from jig_module where jig_id = jig.id) + 1
from jig;

alter table jig drop column cover_id;
alter table jig drop column ending_id;
