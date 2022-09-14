-- kind = 7 (Video)
with cte as (
    select id, array_elems->'Video'->'host'->'Youtube' as youtube_url, ord-1 as item_index
    from jig_data_module module, jsonb_array_elements(
        (
            select contents->'content'->'base'->'stickers'
            from jig_data_module
            where id = module.id
                and kind = 7
        )
    ) with ordinality t(array_elems, ord)
    where kind = 7
        and array_elems#>'{Video}' is not null
)
update jig_data_module
    set contents = jsonb_set(
        contents,
        array['content', 'base', 'stickers', cte.item_index::text, 'Video', 'host', 'Youtube'],
        ('{"url": '||cte.youtube_url::text||'}')::jsonb
    )
from cte
where cte.id = jig_data_module.id
;
