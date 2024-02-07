alter table jig_data_module
    add column stable_id uuid not null default uuid_generate_v1mc();
alter table playlist_data_module
    add column stable_id uuid not null default uuid_generate_v1mc();
alter table resource_data_module
    add column stable_id uuid not null default uuid_generate_v1mc();
alter table course_data_module
    add column stable_id uuid not null default uuid_generate_v1mc();
