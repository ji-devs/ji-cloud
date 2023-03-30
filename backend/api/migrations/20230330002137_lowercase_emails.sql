-- lowercase any uppercase emails
UPDATE user_email
SET email = LOWER(email)
WHERE email <> LOWER(email);

UPDATE user_auth_basic
SET email = LOWER(email)
WHERE email <> LOWER(email);

