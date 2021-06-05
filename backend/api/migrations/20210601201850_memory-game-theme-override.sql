-- Updates MemoryGame's representation on the database.
-- See commits `89160e06b08e8dd5db779ee174b3db1461a57f0a`,
--             `c06cbd8cf736e23e28d0828c2439947926760e5b`,
--             `a1a28ee30d4cebe5a5a1eeee9e6e259ad0a795fa`

select * from jig_module for update;

drop trigger set_updated_at on jig_module;

-- Make set null mode fields to a temporary default value "Duplicate"
update jig_module
set contents = contents - 'mode' || '{"mode": "Duplicate"}'::jsonb
where contents -> 'mode' is null and kind = 3;

-- Update image card structure from `"Image": [ <id>, <lib> ]` to `"Image": { "id": <id>, "lib": <lib> }`
with curr_cards (id, card1, card2, updated_at) as (
    select id,
           jsonb_array_element(pairs, 0) as card1,
           jsonb_array_element(pairs, 1) as card2,
           updated_at
    from jig_module,
         jsonb_array_elements(contents -> 'pairs') as pairs
    where jsonb_array_length(contents -> 'pairs') > 0 and kind = 3 -- exists a pair
),
     updated (id, new_pairs) as (
         select id,
                jsonb_build_array(
                        case
                            when not card1 ? 'Image' then card1
                            else jsonb_build_object(
                                    'Image',
                                    jsonb_build_object(
                                            'id',
                                            jsonb_array_element((card1 ->> 'Image')::jsonb, 0),
                                            'lib',
                                            jsonb_array_element((card1 ->> 'Image')::jsonb, 1)
                                        )
                                )
                            end,
                        case
                            when not card2 ? 'Image' then card2
                            else jsonb_build_object(
                                    'Image',
                                    jsonb_build_object(
                                            'id',
                                            jsonb_array_element((card2 ->> 'Image')::jsonb, 0),
                                            'lib',
                                            jsonb_array_element((card2 ->> 'Image')::jsonb, 1)
                                        )
                                )
                            end
                    ) as new_pairs
         from curr_cards
     ),
     agg as (
         select id,
                jsonb_agg(new_pairs) as new_pairs
         from updated
         group by id
     )
update jig_module
set contents = jsonb_set(contents, '{pairs}', new_pairs),
    updated_at = curr_cards.updated_at
from agg, curr_cards
where agg.id = jig_module.id;

-- push '"theme_id": <value>' down into '"theme": {"Override": <value>}' for all existing memory game entries
with curr (id, contents) as (
    select id,
           contents as memoryGame
    from jig_module
    where kind = 3
)
update jig_module
set contents = curr.contents
                    - 'theme_id'
                    || jsonb_build_object(
                        'theme',
                        jsonb_build_object('Override', curr.contents -> 'theme_id')
                   )
from curr
where curr.id = jig_module.id;

-- Push all objs down from `"{ ... }` into `"{ "content": { ... } }`
update jig_module
set contents = jsonb_build_object(
            'content',
            contents
        )
where kind = 3;

select trigger_updated_at('jig_module');