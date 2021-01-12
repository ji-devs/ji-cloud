create function update_image() returns trigger
    language plpgsql
as
$$
begin
    update image_metadata set updated_at = now() where new.image_id = image_metadata.id or old.image_id = image_metadata.id;
    return null;
end;
$$;

alter table image_metadata add column last_synced_at timestamptz;

create trigger bump_image_updated after insert or delete on image_affiliation for each row execute procedure update_image();
create trigger bump_image_updated after insert or delete on image_style for each row execute procedure update_image();
create trigger bump_image_updated after insert or delete on image_age_range for each row execute procedure update_image();
create trigger bump_image_updated after insert or delete on image_category for each row execute procedure update_image();
