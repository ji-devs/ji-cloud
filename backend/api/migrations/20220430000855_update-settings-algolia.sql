UPDATE settings 
    SET algolia_index_version = 13
WHERE algolia_index_version = 14;

UPDATE course_data 
    SET last_synced_at = NULL
WHERE last_synced_at IS NOT NULL;
