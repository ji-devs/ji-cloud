update jig_data_module
    set is_complete = true
where
    (contents->'content')::text != 'null'
    and jig_data_id in (select id from jig_data where draft_or_live = 1)
    and kind != 11
    and is_complete is false
;
