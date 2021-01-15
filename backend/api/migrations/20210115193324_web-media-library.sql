alter table web_image_library rename to web_media_library;

-- media kind ()
alter table web_media_library add column kind int2 not null;
