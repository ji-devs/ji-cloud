create table web_media_upload (
    media_id uuid primary key references web_media_library(id) on delete restrict,
    uploaded_at timestamptz,
    -- if `uploaded_at is not null and processed_at >= uploaded_at is not true` at, this image hasn't been processed yet.
    processed_at timestamptz,
    -- null if not processed, `is true` if the uploaded was successful, `is not true` otherwise.
    processing_result boolean
);

insert into web_media_upload (media_id, uploaded_at, processed_at, processing_result)
select id as media_id, uploaded_at, now() as processed_at, true as processing_result
from web_media_library;

alter table web_media_library drop column uploaded_at;