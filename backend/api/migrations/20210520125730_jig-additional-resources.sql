create table jig_additional_resource
(
    id     uuid primary key not null default uuid_generate_v1mc(),
    jig_id uuid             not null references jig (id) on delete cascade,
    url    text             not null
);
