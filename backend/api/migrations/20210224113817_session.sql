create table session (
    token           text                 primary key,
    user_id         uuid        not null references "user" (id) on delete cascade,
    impersonator_id uuid                 references "user" (id) on delete cascade,
    created_at      timestamptz not null default now(),
    expires_at      timestamptz,
    -- null is      "unrestricted"
    scope           int2
);
