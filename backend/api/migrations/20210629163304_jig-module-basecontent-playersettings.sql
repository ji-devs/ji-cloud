-- push all objs in 'content' down into 'base'
update jig_module
set contents = jsonb_build_object(
        'content',
        jsonb_build_object(
                'base',
                contents -> 'content'
            )
    )
where kind = any (array [0, 3, 4, 5])               -- Memory, TappingBoard, Poster, or Cover
  and (contents -> 'content' is distinct from null) -- not `postgres null`
  and not (contents -> 'content' ? 'base')          -- base doesn't exist yet
  and not (contents -> 'content' = 'null'::jsonb);
-- there is no content

-- add `player_settings` to `Memory`
update jig_module
set contents =
        jsonb_set(contents,
                  '{content}',
                  contents -> 'content' ||
                  '{
                    "player_settings": {
                      "time_limit": null
                    }
                  }'::jsonb
            )
where kind = 3
  and (contents -> 'content' is distinct from null)   -- not `postgres null`. shouldn't exist but just in case
  and not (contents -> 'content' ? 'player_settings') -- already has the `player_settings` field
  and not (contents -> 'content' = 'null'::jsonb); -- not `json null`
