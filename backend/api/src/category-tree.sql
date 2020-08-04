create or replace temp recursive view category_tree(index, id, parent_id, structure) as
(
select index::int2,
       id,
       parent_id,
       json_build_object('id', id, 'name', name, 'created_at', created_at, 'updated_at', updated_at)
from category co
where not (select exists(select 1 from category ci where ci.parent_id = co.id))
union all
select co.index::int2,
       id,
       parent_id,
       json_build_object('id', id, 'name', name, 'created_at', created_at, 'updated_at', updated_at, 'children',
                         json_agg(_lat.structure))
from category co
         join lateral (
    select ct.structure
    from category_tree ct
    where ct.parent_id = co.id
    order by ct.index
    ) _lat on true
group by co.id
);
