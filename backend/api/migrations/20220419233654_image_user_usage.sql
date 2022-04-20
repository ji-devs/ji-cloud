create table image_usage (
    image_id                uuid                                  not null
         references image_metadata(id)
         on delete cascade,

    -- timestamp which 
    reset_at              timestamp         default now()       not null
);

alter table image_metadata 


create trigger update_path_like
    after insert
    on learning_path_like
    for each row
execute procedure update_imageh_like();

create function update_image_usage() returns trigger
    language plpgsql
as
$$
begin
    update learning_path
    set liked_count = liked_count - 1
    where id = OLD.learning_path_id;
    return NULL;
end;
$$;      