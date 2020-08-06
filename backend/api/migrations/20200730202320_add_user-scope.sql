create table "user_scope"
(
    user_id    uuid references "user" (id) not null,
    scope      int2                        not null,
    created_at timestamptz                 not null default now(),
    unique (user_id, scope)
);
