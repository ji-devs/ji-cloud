-- Add migration script here
alter table image_tag drop constraint image_tag_index_key;
alter table image_tag add constraint image_tag_index_key unique (index) deferrable initially deferred;
