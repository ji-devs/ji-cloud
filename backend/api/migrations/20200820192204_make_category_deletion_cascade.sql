alter table category drop constraint category_parent_id_fkey;
alter table category add constraint category_parent_id_fkey foreign key (parent_id) references category(id) on delete cascade;

alter table image_category drop constraint image_category_category_id_fkey;
alter table image_category add constraint image_category_category_id_fkey foreign key (category_id) references category(id) on delete cascade;
