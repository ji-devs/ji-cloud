-- Add migration script here
create table "content_type"
(
    content_type_id   uuid primary key not null default uuid_generate_v1mc(),
    display_name      text             not null,
    created_at        timestamptz      not null,
    updated_at        timestamptz
);

create table "jig_content_type"
(
    jig_id          uuid references "jig" (id)                       not null,
    content_type_id uuid references "content_type" (content_type_id) not null,
    created_at      timestamptz                                      not null default now(),
    unique          (jig_id, content_type_id)
);

