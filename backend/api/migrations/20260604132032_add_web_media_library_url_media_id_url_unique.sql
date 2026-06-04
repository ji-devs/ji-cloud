alter table web_media_library_url
add constraint web_media_library_url_media_id_media_url_key
unique (media_id, media_url);
