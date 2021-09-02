alter table image_style drop constraint image_style_image_id_fkey;
alter table image_style add constraint image_style_image_id_fkey foreign key (image_id) references image_metadata(id) on delete cascade;

alter table image_age_range drop constraint image_age_range_image_id_fkey;
alter table image_age_range add constraint image_age_range_image_id_fkey foreign key (image_id) references image_metadata(id) on delete cascade;

alter table image_affiliation drop constraint image_affiliation_image_id_fkey;
alter table image_affiliation add constraint image_affiliation_image_id_fkey foreign key (image_id) references image_metadata(id) on delete cascade;

alter table jig_goal drop constraint jig_goal_jig_id_fkey;
alter table jig_goal add constraint jig_goal_jig_id_fkey foreign key (jig_id) references jig(id) on delete cascade;
