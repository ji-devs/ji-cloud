create table user_color (
    user_id uuid not null references "user" (id) on delete cascade,
    "index" int2  not null,
    color   int4  not null,
    unique (user_id, "index")
);
