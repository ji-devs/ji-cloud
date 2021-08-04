alter table "user_profile"
    add column profile_image text default null;

alter table user_image_library
    add column kind smallint not null default 1; -- sticker
