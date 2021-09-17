with recursive path(id, index, parent_id) as (
    select id, ord, null::uuid
    from category
             inner join unnest(
            $1::uuid[]) with ordinality t(id, ord)
                        using (id)
    union all
    select c.id, c.index, p.id
    from path p
             inner join category c on (c.parent_id = p.id)
)
select distinct id as "id!",
       path.index::int2 as "index!",
       path.parent_id,
       name,
       created_at,
       updated_at,
       (select count(*) from image_category where category_id = id)::int8 as "image_count!",
       (select count(*)::int8 from jig_category where category_id = id) as "jig_count!",
       user_scopes

from path
         inner join category using (id);
