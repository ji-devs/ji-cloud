-- create table for Course modules
create table course_data_module
(
    id          uuid                      default uuid_generate_v1mc() not null
        primary key,
    index       smallint                                              not null
        constraint course_data_module_index_check
            check (index = 0),
    course_data_id uuid                                               not null
        references course_data
            on delete cascade,
    kind        smallint                                              not null,
    is_complete boolean                  default false                not null,
    contents    jsonb                    default '{}'::jsonb          not null,
    created_at  timestamp with time zone default now()                not null,
    updated_at  timestamp with time zone default now()                not null,
    unique (index, course_data_id)
);

-- Stable ID is no longer user
alter table jig_data_module 
    drop column stable_id;

-- Create index for Course modules
create index course_data_module_course_data_id_idx
    on course_data_module (course_data_id); 

