alter table global_animation_upload
    drop constraint if exists global_animation_upload_animation_id_fkey;
alter table global_animation_upload
    add constraint global_animation_upload_animation_id_fkey foreign key (animation_id) references animation_metadata(id) on delete restrict;
