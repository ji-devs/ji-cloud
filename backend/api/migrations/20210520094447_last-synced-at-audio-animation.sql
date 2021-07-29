create function update_animation() returns trigger
    language plpgsql
as
$$
begin
    update animation_metadata
    set updated_at = now()
    where new.animation_id = animation_metadata.id
       or old.animation_id = animation_metadata.id;
    return null;
end;
$$;

create trigger bump_animation_updated
    after insert or delete
    on animation_style
    for each row
execute procedure update_animation();


create function update_audio() returns trigger
    language plpgsql
as
$$
begin
    update audio_metadata
    set updated_at = now()
    where new.audio_id = audio_metadata.id
       or old.audio_id = audio_metadata.id;
    return null;
end;
$$;

create trigger bump_audio_updated
    after insert or delete
    on audio_style
    for each row
execute procedure update_audio();
