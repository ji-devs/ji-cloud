alter table jig_data add column if not exists privacy_level smallint not null default 1;

update jig_data 
set privacy_level = jig.privacy_level
from jig 
where jig.id = jig_data.id;

alter table jig drop column privacy_level;
