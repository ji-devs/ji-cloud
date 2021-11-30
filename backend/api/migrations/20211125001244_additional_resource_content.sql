-- types of resources that can be added to a jig
create table resource_type
(
    id  uuid primary key              default uuid_generate_v1mc(),
    display_name text                          not null,              --type of resource  
    created_at   timestamptz                   not null default now(),
    updated_at   timestamptz,   
    index        int2                          check("index" >= 0) unique,
    is_deleted   bool                          not null default false --flag if resource type is removed
);

alter table jig_data_additional_resource 
    add display_name              text             not null, 
    add column resource_type_id   uuid             references "resource_type"(id) not null,
    add column resource_content   jsonb            not null default '{}'::jsonb,
    add unique (resource_type_id, resource_content),
    drop column "url";



