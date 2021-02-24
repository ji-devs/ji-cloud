create table locale_bundle (
    id           uuid        primary key default uuid_generate_v1mc(),
    display_name text        not null,
    created_at   timestamptz not null default now(),
    updated_at   timestamptz
);

create table locale_item_kind (
    id           uuid        primary key default uuid_generate_v1mc(),
    display_name text        not null,
    created_at   timestamptz not null default now(),
    updated_at   timestamptz
);

create table locale_entry (
    id                serial      primary key,
    bundle_id         uuid        not null references locale_bundle(id) on delete cascade,
    section           text,
    item_kind_id      uuid                 references locale_item_kind(id) on delete set null,
    english           text,
    hebrew            text,
                   -- EntryStatus
    status            int2        not null,
    zeplin_reference  text,
    comments          text,
    in_app            boolean     not null, 
    in_element        boolean     not null, 
    in_mock           boolean     not null,
    created_at        timestamptz not null default now(),
    updated_at        timestamptz
);

