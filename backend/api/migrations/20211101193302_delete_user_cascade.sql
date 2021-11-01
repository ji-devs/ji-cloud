alter table jig
    drop constraint jig_author_id_fkey;

alter table jig
    add constraint jig_author_id_fkey
    foreign key (author_id)
    references "user"(id) on delete cascade;

alter table jig
    drop constraint jig_creator_id_fkey;

alter table jig
    add constraint jig_creator_id_fkey
    foreign key (creator_id)
    references "user"(id) on delete cascade;

alter table user_affiliation
    drop constraint user_affiliation_user_id_fkey;

alter table user_affiliation
    add constraint user_affiliation_user_id_fkey
    foreign key (user_id)
    references "user"(id) on delete cascade;

alter table user_age_range
    drop constraint user_age_range_user_id_fkey;

alter table user_age_range
    add constraint user_age_range_user_id_fkey
    foreign key (user_id)
    references "user"(id) on delete cascade;

alter table user_profile
    drop constraint user_profile_user_id_fkey;

alter table user_profile
    add constraint user_profile_user_id_fkey
    foreign key (user_id)
    references "user"(id) on delete cascade;

alter table user_scope
    drop constraint user_scope_user_id_fkey;

alter table user_scope
    add constraint user_scope_user_id_fkey
    foreign key (user_id)
    references "user"(id) on delete cascade;

alter table user_subject
    drop constraint user_subject_user_id_fkey;

alter table user_subject
    add constraint user_subject_user_id_fkey
    foreign key (user_id)
    references "user"(id) on delete cascade;

alter table user_image_upload
    drop constraint user_image_upload_image_id_fkey;

alter table user_image_upload
    add constraint user_image_upload_image_id_fkey
    foreign key (image_id)
    references user_image_library(id) on delete cascade;

alter table user_profile
    drop constraint user_profile_profile_image_id_fkey;

alter table user_profile
    add constraint user_profile_profile_image_id_fkey
    foreign key (profile_image_id)
    references user_image_library(id) on delete cascade;