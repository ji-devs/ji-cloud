-- Add migration script here

alter table circle
drop constraint circle_image_fkey;

alter table circle
add constraint circle_image_fkey foreign key (image)
references user_image_upload(image_id)
on delete cascade;
