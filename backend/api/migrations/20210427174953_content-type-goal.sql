-- Add migration script here
alter table content_type rename to "goal";

alter table goal rename column content_type_id to "id";

alter table goal rename constraint content_type_index_check to "goal_index_check";
alter table goal rename constraint content_type_index_key to "goal_index_key";
alter table goal rename constraint content_type_pkey to "goal_pkey";

alter table jig_content_type rename to "jig_goal";
alter table jig_goal rename column content_type_id to "goal_id";
alter table jig_goal rename constraint jig_content_type_jig_id_content_type_id_key to "jig_goal_jig_id_goal_id_key";
alter table jig_goal rename constraint jig_content_type_content_type_id_fkey to "jig_goal_goal_id_fkey";
alter table jig_goal rename constraint jig_content_type_jig_id_fkey to "jig_goal_jig_id_fkey";

