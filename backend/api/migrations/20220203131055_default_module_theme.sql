-- Remove "Override" in theme
select * from jig_data_module for update;

-- Grab contents of override and push them to theme level
-- '{"a":1,"b":2}'::json->>'b'
-- json_extract_path_text('{"f2":{"f3":1},"f4":{"f5":99,"f6":"foo"}}','f4', 'f6') (result: foo)
with curr_theme (id, theme, updated_at) as (
    select jig_data_module.id               as id,
        jsonb_extract_path_text(contents, 'content', 'base', 'theme', 'Override') as theme,
        jig_data_module.updated_at  as updated_at
    from jig_data_module
    inner join jig_data on jig_data.id = jig_data_module.jig_data_id
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

-- replace all "{"theme": "Jig"}" with "{"theme": <value>}"
with curr (module_id, theme_name, jig_theme, updated_at) as (
    select jig_data_module.id           as module_id,
        theme.display_name as theme_name,
        jig_data.theme as jig_theme,
        jig_data_module.updated_at      as updated_at
    from jig_data_module
    inner join jig_data on jig_data.id = jig_data_module.jig_data_id
    inner join theme on theme.index = jig_data.theme
    where contents -> 'content' -> 'base' ? 'theme' and contents -> 'content' -> 'base' ->> 'theme' = 'Jig'
)
update jig_data_module
set contents = jsonb_set(contents, '{content, base, theme}', to_jsonb(theme_name), false),
    updated_at = curr.updated_at
from
    curr
where curr.module_id = jig_data_module.id;
