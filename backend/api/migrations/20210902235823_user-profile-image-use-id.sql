alter table user_profile
    drop column profile_image,
    add column profile_image_id uuid references user_image_library (id);
