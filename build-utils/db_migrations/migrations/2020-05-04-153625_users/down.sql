--- not reverting the empty arrays...

ALTER TABLE users
    ALTER COLUMN roles DROP NOT NULL;