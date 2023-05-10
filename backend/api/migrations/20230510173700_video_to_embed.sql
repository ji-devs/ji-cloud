-- kind = 7 (Video)
-- need to use COALESCE to get 'null' string on optional filed because all expressions that involves a null, yields null. (https://stackoverflow.com/a/34940667/5253155)

with cte as (
    select
        id,
        ord-1 as sticker_index,
        sticker->'Video'->'host'->'Youtube'->'url' as url,
        sticker->'Video'->'start_at' as start_at,
        sticker->'Video'->'end_at' as end_at,
        sticker->'Video'->'transform' as transform,
        contents->'content'->'play_settings'->'muted' as muted,
        contents->'content'->'play_settings'->'autoplay' as autoplay,
        contents->'content'->'play_settings'->'captions' as captions,
        contents->'content'->'play_settings'->'done_action' as done_action
    from jig_data_module module, jsonb_array_elements(
        (
            select contents->'content'->'base'->'stickers'
            from jig_data_module
            where id = module.id
                and kind = 7
        )
    ) with ordinality t(sticker, ord)
    where kind = 7
        and sticker#>'{Video}' is not null
)
update jig_data_module
    set contents = jsonb_set(
        contents,
        array['content', 'base', 'stickers', cte.sticker_index::text],
        ('
            {
                "Embed": {
                    "host": {
                        "Youtube": {
                            "url": '||cte.url::text||',
                            "done_action": '||cte.done_action::text||',
                            "start_at": '||COALESCE(cte.start_at::text, 'null')||',
                            "end_at": '||COALESCE(cte.end_at::text, 'null')||',
                            "muted": '||cte.muted::text||',
                            "autoplay": '||cte.autoplay::text||',
                            "captions": '||cte.captions::text||'
                        }
                    },
                    "transform": '||cte.transform::text||'
                }
            }
        ')::jsonb
    )

from cte
where cte.id = jig_data_module.id
;
-- Add migration script here
-- kind = 7 (Video)

with cte as (
    select
        id,
        ord-1 as sticker_index,
        sticker->'Video'->'host'->'Youtube'->'url' as url,
        sticker->'Video'->'start_at' as start_at,
        sticker->'Video'->'end_at' as end_at,
        sticker->'Video'->'transform' as transform,
        contents->'content'->'play_settings'->'muted' as muted,
        contents->'content'->'play_settings'->'autoplay' as autoplay,
        contents->'content'->'play_settings'->'captions' as captions,
        contents->'content'->'play_settings'->'done_action' as done_action
    from jig_data_module module, jsonb_array_elements(
        (
            select contents->'content'->'base'->'stickers'
            from jig_data_module
            where id = module.id
                and kind = 7
        )
    ) with ordinality t(sticker, ord)
    where kind = 7
        and sticker#>'{Video}' is not null
)
update jig_data_module
    set contents = jsonb_set(
        contents,
        array['content', 'base', 'stickers', cte.sticker_index::text],
        ('
            {
                "Embed": {
                    "host": {
                        "Youtube": {
                            "url": '||cte.url::text||',
                            "done_action": '||cte.done_action::text||',
                            "start_at": '||COALESCE(cte.start_at::text, 'null')||',
                            "end_at": '||COALESCE(cte.end_at::text, 'null')||',
                            "muted": '||cte.muted::text||',
                            "autoplay": '||cte.autoplay::text||',
                            "captions": '||cte.captions::text||'
                        }
                    },
                    "transform": '||cte.transform::text||'
                }
            }
        ')::jsonb
    )

from cte
where cte.id = jig_data_module.id
;
