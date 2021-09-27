alter table user_profile
    add column profile_image_id uuid references user_image_library (id),
    drop column profile_image;
