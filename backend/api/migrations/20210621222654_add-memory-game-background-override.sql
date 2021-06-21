update jig_module
set contents =
        jsonb_set(contents,
                  '{content}',
                  contents -> 'content' ||
                  '{"background": null}'::jsonb
            )
where contents -> 'content' is distinct from null -- not `postgres null`. shouldn't exist but just in case
  and not contents -> 'content' ? 'editor_state' -- already has the `editor_state` field
  and not contents -> 'content' = 'null'::jsonb -- not `json null`
  and kind = 3;
