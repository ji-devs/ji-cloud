alter table jig
    add column audio_background        smallint, -- null allowed
    add column audio_feedback_negative smallint[] not null default '{}',
    add column audio_feedback_positive smallint[] not null default '{}';
