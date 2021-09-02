with del_ids as (
    select id
    from image_metadata
             left join image_upload on id = image_id
    where created_at < (now() - interval '3 day')
      and processed_at is not distinct from null
),
     del_uploads as (
         delete from image_upload where image_id in (select id from del_ids)
     )
delete
from image_metadata
where id in (select id from del_ids);

with del_ids as (
    select id
    from user_image_library
             left join user_image_upload on id = image_id
    where created_at < (now() - interval '3 day')
      and processed_at is not distinct from null
),
     del_uploads as (
         delete from user_image_upload where image_id in (select id from del_ids)
     )
delete
from user_image_library
where id in (select id from del_ids);

with del_ids as (
    select id
    from user_audio_library
             left join user_audio_upload on id = audio_id
    where created_at < (now() - interval '3 day')
      and processed_at is not distinct from null
),
     del_uploads as (
         delete from user_audio_upload where audio_id in (select id from del_ids)
     )
delete
from user_audio_library
where id in (select id from del_ids);

with del_ids as (
    select id
    from animation_metadata
             left join global_animation_upload on id = animation_id
    where created_at < (now() - interval '3 days')
      and processed_at is not distinct from null
),
     del_uploads as (
         delete from global_animation_upload where animation_id in (select id from del_ids)
     )
delete
from animation_metadata
where id in (select id from del_ids);
