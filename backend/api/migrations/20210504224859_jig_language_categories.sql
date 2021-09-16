create table jig_category (
    jig_id      uuid        not null references jig(id) on delete cascade,
    category_id uuid        not null references category(id) on delete cascade,
    created_at  timestamptz not null default now()
);

alter table jig add column language text not null default 'en';

alter table jig alter column language drop default;
