-- Update all instances of {SelectSome: n} to Continue
update jig_data_module
set contents = jsonb_set(
    contents,
    '{content,play_settings,next}',
    to_jsonb(('Continue')::TEXT)
)
where
    kind=5
    and contents->'content'->'play_settings'->'next' @? '$ ? (exists (@."SelectSome"))'
;
