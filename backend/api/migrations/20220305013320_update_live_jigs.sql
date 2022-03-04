-- Update jigs in algolia with new field blocked
update jig_data
set updated_at = now()
where last_synced_at is not null and draft_or_live = 1;
