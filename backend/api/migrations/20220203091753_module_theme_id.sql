-- Jigzi Theme ids
create table theme (
    display_name         text           not null,
    index                smallint       unique not null check (index >= 0),
    created_at           timestamptz not null    default now(),
    updated_at           timestamptz
);
