-- Rename ThemeId
update jig_module
set contents = jsonb_set(
    contents,
    '{content,theme,Override}',
    '{"Override": "Blank"}'::jsonb
)
where contents->'content'->'theme' ? 'Override';
