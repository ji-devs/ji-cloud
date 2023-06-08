ALTER TABLE user_profile ALTER COLUMN location_public SET DEFAULT true;
ALTER TABLE user_profile ALTER COLUMN languages_spoken_public SET DEFAULT true;

UPDATE user_profile SET location_public = true, languages_spoken_public = true;
