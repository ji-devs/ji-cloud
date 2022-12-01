-- Convert timer setting to seconds from minutes

-- Update where module uses `play_settings`
-- select contents->'content'->'play_settings'->>'time_limit' from jig_data_module where contents->'content'->'play_settings'->>'time_limit' IS NOT NULL;
update jig_data_module
set contents = jsonb_set(
    contents,
    '{content,play_settings,time_limit}',
    to_jsonb((contents->'content'->'play_settings'->>'time_limit')::INTEGER * 60)
)
;

-- Update where module uses `player_settings`
-- select contents->'content'->'player_settings'->>'time_limit' from jig_data_module where contents->'content'->'player_settings'->>'time_limit' IS NOT NULL;
update jig_data_module
set contents = jsonb_set(
    contents,
    '{content,player_settings,time_limit}',
    to_jsonb((contents->'content'->'player_settings'->>'time_limit')::INTEGER * 60)
)
;
