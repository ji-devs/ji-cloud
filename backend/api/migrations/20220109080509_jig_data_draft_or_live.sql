alter table jig_data 
    add column draft_or_live int2;

update jig_data 
set draft_or_live = 0
from jig
where jig.draft_id = jig_data.id;

update jig_data 
set draft_or_live = 1
from jig
where jig.live_id = jig_data.id;
