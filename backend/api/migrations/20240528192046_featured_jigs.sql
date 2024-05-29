create table featured_jigs
(
    id         uuid primary key default uuid_generate_v1mc(),
    jig_id     uuid not null references jig (id) on delete cascade,
    index      int not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz
);
