-- push '"theme_id": <value>' down into '"theme": {"Override": <value>}' for all existing memory game entries
with curr (id, memoryGame) as (
    select
        id,
        contents -> 'memoryGame' as memoryGame
    from jig_module
    where contents ? 'memoryGame'
    for update
)
update jig_module
set contents = jsonb_insert(
        contents,
        '{memoryGame}',
        curr.memoryGame
            - 'theme_id'
            || jsonb_build_object(
                'theme',
                jsonb_build_object('Override', curr.memoryGame -> 'theme_id')
        )
)
from curr
where curr.id = jig_module.id;