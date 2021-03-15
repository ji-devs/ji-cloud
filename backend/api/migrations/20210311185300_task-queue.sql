create table image_upload (
    image_id uuid primary key references image_metadata(id) on delete restrict,
    uploaded_at timestamptz,
    -- if `uploaded_at is not null and processed_at >= uploaded_at is not true` at, this image hasn't been processed yet.
    processed_at timestamptz,
    -- null if not processed, `is true` if the uploaded was successful, `is not true` otherwise.
    processing_result boolean
);

insert into image_upload (image_id, uploaded_at, processed_at, processing_result)
select id as image_id, uploaded_at, now() as processed_at, true as processing_result
from image_metadata;

alter table image_metadata drop column uploaded_at;

create table user_image_upload (
    image_id uuid primary key references image_metadata(id) on delete restrict,
    uploaded_at timestamptz,
    -- if `uploaded_at is not null and processed_at >= uploaded_at is not true` at, this image hasn't been processed yet.
    processed_at timestamptz,
    -- null if not processed, `is true` if the uploaded was successful, `is not true` otherwise.
    processing_result boolean
);

insert into user_image_upload (image_id, uploaded_at, processed_at, processing_result)
select id as image_id, uploaded_at, now() as processed_at, true as processing_result
from user_image_library;

alter table user_image_library drop column uploaded_at;
