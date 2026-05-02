CREATE TABLE user_daily_plays (
    user_id UUID NOT NULL REFERENCES user_profile(user_id) ON DELETE CASCADE,
    play_date DATE NOT NULL DEFAULT CURRENT_DATE,
    play_count INT NOT NULL DEFAULT 1 CHECK (play_count >= 0),
    PRIMARY KEY (user_id, play_date)
);

CREATE INDEX idx_user_daily_plays_user_date ON user_daily_plays(user_id, play_date);
