alter table image_metadata
    add column translated_name  jsonb   default '{}'::jsonb not null;

alter table jig_data
    add column translated_name  jsonb   default '{}'::jsonb not null;

alter table course_data
    add column translated_name  jsonb   default '{}'::jsonb not null;
    