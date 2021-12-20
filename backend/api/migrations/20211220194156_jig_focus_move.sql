alter table jig 
    add column jig_focus     int2           not null    default 0;

update jig 
set jig_focus = (select jig_focus
                 from jig_data 
                 where jig.draft_id = jig_data.id);


alter table jig_data
    drop column jig_focus;