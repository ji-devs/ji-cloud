create table user_follow (
    user_id         uuid              not null
        references "user" (id)           on delete cascade, 

    follower_id         uuid          not null
        references "user" (id)           on delete cascade, 

    followed_at           timestamp,

    unique(user_id, follower_id)
);

