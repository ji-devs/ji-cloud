-- removes all references to `id` image tags, moving to using index::smallint entirely
--

alter table if exists image_tag_join
    rename to old_image_tag_join;

alter table old_image_tag_join
    drop constraint image_tag_join_tag_id_fkey;

alter table image_tag
    drop constraint if exists image_tag_pkey,
    drop constraint if exists image_tag_index_key,
    add constraint image_tag_index_pkey primary key (index);

create table image_tag_join
(
    image_id  uuid     not null references image_metadata (id) on delete cascade,
    tag_index smallint not null references image_tag (index) on delete cascade
);

insert into image_tag_join (image_id, tag_index)
select image_id, index
from old_image_tag_join
         left join image_tag on old_image_tag_join.tag_id = image_tag.id;

drop table old_image_tag_join;

alter table image_tag
    drop column id;
