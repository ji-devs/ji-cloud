create table jig (
    id           uuid primary key       default uuid_generate_v1mc(),
    display_name text          not null,
    cover        jsonb         not null,
    ending       jsonb         not null,
    -- The difference between the creator and the author is that the *creator* cannot be changed, and is a bit of record keeping.
    -- A author is the person currently in charge of the jig, whereas the creator is the person who was *originally* responsible for it.
    creator_id   uuid references "user" (id) on delete set null,
    author_id    uuid references "user" (id) on delete set null,
    created_at   timestamptz   not null default now(),
    updated_at   timestamptz,
    publish_at   timestamptz
);

create table jig_module (
    jig_id uuid  not null references jig (id) on delete cascade,
    module jsonb not null
);
