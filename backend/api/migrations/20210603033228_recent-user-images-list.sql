-- Add migration script here
create table user_recent_image
(
    user_id       uuid references "user" (id) on delete cascade,
    image_id      uuid                        not null,
    media_library smallint                    not null check (media_library >= 0),
    last_used     timestamptz                 not null default now(),
    unique(user_id, image_id)
);

