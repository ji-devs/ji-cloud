alter table image_style drop constraint image_style_style_id_fkey;
alter table image_style add constraint image_style_style_id_fkey foreign key (style_id) references style(id) on delete cascade;

alter table image_age_range drop constraint image_age_range_age_range_id_fkey;
alter table image_age_range add constraint image_age_range_age_range_id_fkey foreign key (age_range_id) references age_range(id) on delete cascade;

alter table image_affiliation drop constraint image_affiliation_affiliation_id_fkey;
alter table image_affiliation add constraint image_affiliation_affiliation_id_fkey foreign key (affiliation_id) references affiliation(id) on delete cascade;

alter table jig_goal drop constraint jig_goal_goal_id_fkey;
alter table jig_goal add constraint jig_goal_goal_id_fkey foreign key (goal_id) references goal(id) on delete cascade;

alter table jig_affiliation drop constraint jig_affiliation_affiliation_id_fkey;
alter table jig_affiliation add constraint jig_affiliation_affiliation_id_fkey foreign key (affiliation_id) references affiliation(id) on delete cascade;
