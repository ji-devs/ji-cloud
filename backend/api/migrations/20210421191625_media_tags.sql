-- couldn't come up with a better name, sorry to whomever has to interact with or extend this
-- the naming scheme is {media_kind}_tag, and `{media_kind}_tag_join`,
-- as compared to the rest of the project which would do something like `tag` and `{media_kind}`_tag respectively
create table image_tag (
    id           uuid        primary key default uuid_generate_v1mc(),
    display_name text        not null,
    created_at   timestamptz not null    default now(),
    updated_at   timestamptz,
    index int2   unique      not null    check (index >= 0)
);

select trigger_updated_at('image_tag');

create table image_tag_join (
    image_id uuid not null references image_metadata (id) on delete cascade,
    tag_id   uuid not null references image_tag      (id) on delete cascade
);

create trigger bump_image_updated after insert or delete on image_tag_join for each row execute procedure update_image();

select trigger_updated_at('image_metadata');
