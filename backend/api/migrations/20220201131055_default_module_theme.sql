-- Remove "Override" in theme
select * from jig_data_module for update;

drop trigger set_updated_at on jig_data_module;

-- Grab contents of override and push them to theme level 
-- '{"a":1,"b":2}'::json->>'b' 
-- json_extract_path_text('{"f2":{"f3":1},"f4":{"f5":99,"f6":"foo"}}','f4', 'f6') (result: foo)
with curr_theme (id, theme, updated_at) as (
    select id,
        jsonb_extract_path_text(contents, 'content', 'base', 'theme', 'Override') as theme,
        updated_at
    from jig_data_module
    where contents -> 'content' -> 'base' -> 'theme' ? 'Override' -- exists override
),
    updated (id, new_themes) as (
        select id,
        theme as new_themes
    from curr_theme
    ),
    agg as (
        select id,
            to_jsonb(new_themes) as new_themes
        from updated
        order by id
    )
update jig_data_module
set contents = jsonb_set(contents, '{content, base, theme}', new_themes, false),
    updated_at = curr_theme.updated_at
from agg,
    curr_theme
where agg.id = jig_data_module.id;

select trigger_updated_at('jig_data_module');
