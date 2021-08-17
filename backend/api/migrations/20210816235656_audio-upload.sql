create table user_audio_upload
(
    audio_id          uuid primary key references user_audio_library (id) on delete restrict,
    uploaded_at       timestamptz,
    -- if `uploaded_at is not null and processed_at >= uploaded_at is not true` at, this audio hasn't been processed yet.
    processed_at      timestamptz,
    -- null if not processed, `is true` if the uploaded was successful, `is not true` otherwise.
    processing_result boolean
);

insert into user_audio_upload (audio_id, uploaded_at, processed_at, processing_result)
select id as audio_id, uploaded_at, now() as processed_at, true as processing_result
from user_audio_library;

alter table user_audio_library
    drop column uploaded_at;
