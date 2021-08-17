insert into animation_metadata (id, name, description, is_premium, created_at, is_looping, kind)
values
    ('3de4bcb6-7d23-11eb-8c8a-fbaee84afb0a', 'test', 'testest', false, '2021-03-04 19:53:08.062618+00', false, 0);

insert into global_animation_upload (animation_id, uploaded_at, processed_at, processing_result)
select id, created_at, created_at + interval '5 minutes', true
from animation_metadata;
