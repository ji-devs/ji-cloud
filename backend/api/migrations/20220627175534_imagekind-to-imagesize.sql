alter table user_image_library
rename column kind to size;

alter table image_metadata
rename column kind to size;
