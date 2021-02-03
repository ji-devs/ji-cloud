-- Add migration script here
alter table web_media_library add column uploaded_at timestamptz;
alter table user_image_library add column uploaded_at timestamptz;
alter table image_metadata add column uploaded_at timestamptz;
alter table animation add column uploaded_at timestamptz;
alter table user_audio_library add column uploaded_at timestamptz;
