ALTER TABLE user_auth_google ADD COLUMN unverified_email citext;
CREATE INDEX idx_user_auth_google_unverified_email ON user_auth_google(unverified_email);
