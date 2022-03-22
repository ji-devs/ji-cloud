with recursive links as
(
    select id,
    parent_id
    from category co
    where id = any ($1::uuid[])
    union all
    select co.id,
    co.parent_id
    from category co
    inner join links ct on (ct.parent_id = co.id)
)

select
    distinct id,
    category.parent_id,
    name,
    category.index,
    created_at,
    updated_at,
    user_scopes
from category
inner join links using (id);
