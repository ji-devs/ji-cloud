alter table image_upload
    add column kind smallint not null default 0;

alter table user_image_upload
    add column kind smallint not null default 0;
