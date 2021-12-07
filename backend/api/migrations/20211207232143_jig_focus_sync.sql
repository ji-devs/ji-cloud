-- sync all jigs with algolia
update jig_data 
set updated_at = now()
where last_synced_at is not null;

