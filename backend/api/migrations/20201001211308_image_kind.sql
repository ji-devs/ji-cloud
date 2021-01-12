-- Add migration script here
truncate table image_metadata cascade;
alter table image_metadata add column kind int2 not null;
