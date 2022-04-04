update jig_data_module
set contents = jsonb_set(contents, '{"content", "editor_state"}', '{"step": "Five", "steps_completed": ["Four", "Three", "Two", "One"]}'::jsonb)
where kind = 10
    and contents->'content'->'editor_state'->>'step' = 'Five';
