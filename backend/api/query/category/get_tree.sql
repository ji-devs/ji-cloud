with recursive category_tree(index, id, parent_id, structure) as
                   (
                       select index::int2,
                              id,
                              parent_id,
                              jsonb_build_object(
                                      'id', id,
                                      'name', name, 'created_at', created_at, 'updated_at',
                                      updated_at,
                                      'image_count',
                                      (select count(*) from image_category where category_id = id)::int8,
                                      'jig_count', 0::int8
                                  )
                       from category co
                       where not (select exists(select 1 from category ci where ci.parent_id = co.id))
                       union all
                       select co.index::int2,
                              id,
                              parent_id,
                              jsonb_build_object('id', id, 'name', name, 'created_at', created_at, 'updated_at',
                                                 updated_at, 'children',
                                                 jsonb_agg(_lat.structure),
                                                 'image_count',
                                                 (select count(*) from image_category where category_id = id)::int8,
                                                 'jig_count', 0::int8)
                       from category co
                                join lateral (
                           select ct.structure
                           from category_tree ct
                           where ct.parent_id = co.id
                           order by ct.index
                           ) _lat on true
                       group by co.id
                   )
select coalesce(jsonb_agg(structure order by index), '[]') as "categories!: sqlx::types::Json<Vec<Category>>"
from category_tree
where parent_id is null
