-- types of resources that can be added to a jig
create table additional_resource
(
    id  uuid primary key              default uuid_generate_v1mc(),
    display_name text                          not null,              --type of resource  
    created_at   timestamptz                   not null default now(),
    is_deleted   bool                          not null default false --flag if resource type is removed
);

alter table jig_data_additional_resource 
    add column resource_id        uuid             references "additional_resource"(id) not null unique,
    add column resource_content   jsonb            not null default '{}'::jsonb,
    drop column "url",
    drop column "id";




