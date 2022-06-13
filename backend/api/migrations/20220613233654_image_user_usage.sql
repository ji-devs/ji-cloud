-- table keeps track of image usage expiration until reset
CREATE TABLE image_usage (
    image_id                uuid                NOT NULL
         references image_metadata(id)
         on delete cascade,

    --Timestamp for initial image usage reset
    usage_reset_at              timestamp with time zone      NOT NULL    DEFAULT now()
);

ALTER TABLE image_metadata
    ADD COLUMN  usage     bigint     NOT NULL       DEFAULT 0;


INSERT INTO image_usage(image_id)
SELECT id
FROM image_metadata;

UPDATE image_metadata
SET last_synced_at = NULL
WHERE last_synced_at IS NOT NULL;


-- Function to reset user usage amount
CREATE FUNCTION set_reset_usage() RETURNS TRIGGER
    language plpgsql
as
$$
BEGIN
    UPDATE image_metadata
    SET user_usage = 0,
        last_synced_at = NULL
    WHERE id = NEW.image_id;
    RETURN NULL;
END;
$$;

CREATE TRIGGER set_reset_timestamp_at
    AFTER UPDATE
    ON image_usage
    FOR EACH ROW
EXECUTE PROCEDURE set_reset_usage();

-- Adds new image_id and timestamp to image usage table
CREATE FUNCTION add_image_usage() RETURNS TRIGGER
    language plpgsql
as
$$
BEGIN
    INSERT INTO image_usage(image_id)
    VALUES(NEW.id);
    RETURN NULL;
END;
$$;

CREATE TRIGGER trigger_add_image_usage
    AFTER INSERT
    ON image_metadata
    FOR EACH ROW
EXECUTE PROCEDURE add_image_usage();
