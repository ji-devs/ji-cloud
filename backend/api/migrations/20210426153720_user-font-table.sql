create table user_font (
    user_id     uuid    not null references "user" (id) on delete cascade,
    "index"     int2    not null,
    font_name   text    not null,
    unique (user_id, "index")
);