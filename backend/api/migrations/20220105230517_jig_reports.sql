create table jig_report (
    id             uuid              primary key    default uuid_generate_v1mc() not null,

    jig_id         uuid              not null       references jig(id) on delete cascade,

    report_type    int2              not null,

    reporter       uuid              references "user"(id),

    created_at     timestamptz   default now() not null;       
);
