-- this is an addition to the previous few migrations
UPDATE public.algolia_index_settings SET index_name = 'playlist_index' WHERE index_name = 'course_index';
UPDATE public.algolia_index_settings SET index_name = 'course_index' WHERE index_name = 'pro_dev_index';
