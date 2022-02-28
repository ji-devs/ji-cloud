create index jig_live_id on jig(live_id);
create index jig_draft_id on jig(draft_id);
create index jig_data_module_jig_data_id_idx on jig_data_module(jig_data_id);
create index jig_data_goal_jig_data_id_idx on jig_data_goal(jig_data_id);
create index jig_data_category_jig_data_id_idx on jig_data_category(jig_data_id);
create index jig_data_affiliation_jig_data_id_idx on jig_data_affiliation(jig_data_id);
create index jig_data_additional_resource_jig_data_id_idx on jig_data_additional_resource(jig_data_id);
create index jig_data_age_range_jig_data_id_idx on jig_data_age_range(jig_data_id);
