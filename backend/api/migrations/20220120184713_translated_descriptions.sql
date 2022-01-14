alter table image_metadata
    drop column translated_description;

alter table image_metadata
    add column translated_description  jsonb   default '{}'::jsonb;

alter table jig_data
    add column translated_description  jsonb   default '{}'::jsonb;
