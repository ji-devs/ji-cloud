

alter table course_data_resource drop constraint learning_path_data_resource_learning_path_data_id_fkey;
alter table course_data_resource add constraint course_data_resource_course_data_id_fkey foreign key (course_data_id) references course_data(id) on delete cascade;

alter table course_data_resource drop constraint learning_path_data_resource_resource_type_id_fkey;
alter table course_data_resource add constraint course_data_resource_resource_type_id_fkey foreign key (resource_type_id) references resource_type(id) on delete cascade;


