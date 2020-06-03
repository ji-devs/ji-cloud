ALTER TABLE users
    ALTER COLUMN roles SET DEFAULT array[]::integer[];

UPDATE users
    SET roles = DEFAULT WHERE roles IS NULL;

ALTER TABLE users
    ALTER COLUMN roles SET NOT NULL;