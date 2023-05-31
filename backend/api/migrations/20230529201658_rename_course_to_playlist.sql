 -- drop pre existing indices
DROP INDEX course_draft_id;
DROP INDEX course_live_id;
DROP INDEX course_data_resource_course_data_id_idx;
DROP INDEX course_data_affiliation_course_data_id_idx;
DROP INDEX course_data_age_range_course_data_id_idx;
DROP INDEX course_data_category_course_data_id_idx;
DROP INDEX course_data_module_course_data_id_idx;






-- drop pre existing functions, that will intern drop the triggers
drop function bump_course_data_updated_at cascade;
drop function course_update_live_up_to_date cascade;
drop function course_publish_live_up_to_date cascade;
drop function course_translate_des_status cascade;
drop function course_translate_name_status cascade;
drop function update_course_like cascade;
drop function update_course_unlike cascade;
drop function process_jig_index cascade;




-- rename tables
ALTER TABLE course_data_module RENAME TO playlist_data_module;
ALTER TABLE course_data RENAME TO playlist_data;
ALTER TABLE course_data_jig RENAME TO playlist_data_jig;
ALTER TABLE course RENAME TO playlist;
ALTER TABLE course_data_category RENAME TO playlist_data_category;
ALTER TABLE course_data_age_range RENAME TO playlist_data_age_range;
ALTER TABLE course_data_affiliation RENAME TO playlist_data_affiliation;
ALTER TABLE course_data_resource RENAME TO playlist_data_resource;
ALTER TABLE course_like RENAME TO playlist_like;

-- rename columns
ALTER TABLE playlist_data_module RENAME COLUMN course_data_id TO playlist_data_id;
ALTER TABLE playlist_data_jig RENAME COLUMN course_data_id TO playlist_data_id;
ALTER TABLE playlist_data_category RENAME COLUMN course_data_id TO playlist_data_id;
ALTER TABLE playlist_data_age_range RENAME COLUMN course_data_id TO playlist_data_id;
ALTER TABLE playlist_data_affiliation RENAME COLUMN course_data_id TO playlist_data_id;
ALTER TABLE playlist_data_resource RENAME COLUMN course_data_id TO playlist_data_id;
ALTER TABLE playlist_like RENAME COLUMN course_id TO playlist_id;





-- rename constraint
alter table playlist_data_resource drop constraint course_data_resource_course_data_id_fkey;
alter table playlist_data_resource add constraint playlist_data_resource_playlist_data_id_fkey foreign key (playlist_data_id) references playlist_data(id) on delete cascade;

alter table playlist_data_resource drop constraint course_data_resource_resource_type_id_fkey;
alter table playlist_data_resource add constraint playlist_data_resource_resource_type_id_fkey foreign key (resource_type_id) references resource_type(id) on delete cascade;

alter table playlist_data_jig drop constraint course_data_jig_course_data_id_jig_id_index_key;
alter table playlist_data_jig add constraint playlist_data_jig_playlist_data_id_jig_id_index_key unique(playlist_data_id, jig_id, index);




-- rename indexes
create index playlist_draft_id
    on playlist (draft_id);
create index playlist_live_id
    on playlist (live_id);
create index playlist_data_resource_playlist_data_id_idx
    on playlist_data_resource (playlist_data_id);
create index playlist_data_affiliation_playlist_data_id_idx
    on playlist_data_affiliation (playlist_data_id);
create index playlist_data_age_range_playlist_data_id_idx
    on playlist_data_age_range (playlist_data_id);
create index playlist_data_category_playlist_data_id_idx
    on playlist_data_category (playlist_data_id);
create index playlist_like_playlist_id_idx
    on playlist_like (playlist_id);
create index playlist_data_module_playlist_data_id_idx
    on playlist_data_module (playlist_data_id);



-- recreate functions and trigger for playlists
create function bump_playlist_data_updated_at() returns trigger
    language plpgsql
as
$$
begin
    update playlist_data set updated_at = now() where new.playlist_data_id = playlist_data.id or old.playlist_data_id = playlist_data.id;
    return null;
end;
$$;

create trigger bump_playlist_data_updated_at after insert or delete on playlist_data_age_range for each row execute procedure bump_playlist_data_updated_at();
create trigger bump_playlist_data_updated_at after insert or delete on playlist_data_affiliation for each row execute procedure bump_playlist_data_updated_at();
create trigger bump_playlist_data_updated_at after insert or delete on playlist_data_category for each row execute procedure bump_playlist_data_updated_at();
create trigger bump_playlist_data_updated_at after insert or delete on playlist_data_jig for each row execute procedure bump_playlist_data_updated_at();


create function playlist_update_live_up_to_date() returns trigger
    language plpgsql
    as
$$
    declare
        published_at timestamptz := (select published_at from playlist where playlist.draft_id = new.id or playlist.live_id = new.id);
    begin
        update playlist
        set live_up_to_date = false
        where playlist.draft_id = new.id;
        return null;
    END;
$$;

create trigger playlist_update_on_draft
    after update of updated_at
    on playlist_data
    for each row
execute procedure playlist_update_live_up_to_date();


create function playlist_publish_live_up_to_date() returns trigger
    language plpgsql
    as
$$
    begin
        update playlist
        set live_up_to_date = true
        where playlist.id = new.id and old.live_up_to_date = false;
        return null;
    END;
$$;

create trigger playlist_update_on_publish
    after update of published_at
    on playlist
    for each row
execute procedure playlist_publish_live_up_to_date();


create function playlist_translate_des_status() returns trigger
    language plpgsql
as
$$
begin
    update playlist
    set description_translate_status = null
    where live_id = new.id;
    return NEW;
end;
$$;

create trigger playlist_translate_des_status
    after update of description
    on playlist_data
    for each row
execute procedure playlist_translate_des_status();


create function playlist_translate_name_status() returns trigger
    language plpgsql
as
$$
begin
    update playlist
    set name_translate_status = null
    where live_id = new.id;
    return NEW;
end;
$$;

create trigger playlist_translate_name_status
    after update of display_name
    on playlist_data
    for each row
execute procedure playlist_translate_name_status();


create function update_playlist_like() returns trigger
    language plpgsql
as
$$
begin
    update playlist
    set likes = likes + 1
    where id = NEW.playlist_id;
    return NEW;
end;
$$;

create trigger add_playlist_like
    after insert
    on playlist_like
    for each row
execute procedure update_playlist_like();


create function update_playlist_unlike() returns trigger
    language plpgsql
as
$$
begin
    update playlist
    set likes = likes - 1
    where id = OLD.playlist_id;
    return NULL;
end;
$$;

create trigger playlist_unlike
    after delete
    on playlist_like
    for each row
execute procedure update_playlist_unlike();


create function process_jig_index() returns trigger
    language plpgsql
    as
$$
    begin
    create table temp_playlist_jig (
        playlist_data_id      uuid,
        jig_id    uuid,
        index    smallint
    );

    insert into temp_playlist_jig
    select
      playlist_data_id,
      jig_id,
      row_number () over (
        partition by playlist_data_id
          order by index
      ) - 1 as index
      from playlist_data_jig cdj
      where cdj.index is null;

    update playlist_data_jig
      set index = subquery.index
    from (select playlist_data_id, jig_id, index from temp_playlist_jig) as subquery
    where playlist_data_jig.playlist_data_id = subquery.playlist_data_id and playlist_data_jig.jig_id = subquery.jig_id;

    drop table temp_playlist_jig;

    return null;
    end;
$$;

-- "for each statement" instead of "for each row"
create trigger jig_index
after insert on playlist_data_jig
    for each statement
execute procedure process_jig_index();
