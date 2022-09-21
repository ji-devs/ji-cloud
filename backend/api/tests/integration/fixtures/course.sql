insert into 
        course_data (id, display_name, created_at, 
                updated_at, language, last_synced_at, description,  privacy_level, draft_or_live, translated_description)
values ('566ed564-f3ec-11ec-b8ef-93b44835e7e4', 'course name1', '2021-03-04 00:46:26.134651+00', -- live
        '2021-03-04 00:46:26.134651+00', 'en', '2021-03-04 00:46:26.134651+00', 'live test description', 0, 1, '{}'::jsonb),
       ('db817a2c-f3ec-11ec-b8ef-37027ca902ea', 'course name2', '2021-03-04 00:46:26.134651+00', -- live
        '2021-03-04 00:46:26.134651+00', 'en', '2021-03-04 00:46:26.134651+00', 'live test description', 0, 1, '{}'::jsonb),
       ('f393863c-f3ec-11ec-b8ef-43660dc59560', 'course name3', '2021-03-04 00:46:26.134651+00', -- live
        '2021-03-04 00:46:26.134651+00', 'en', '2021-03-04 00:46:26.134651+00', 'live test description', 0, 1, '{}'::jsonb),
       ('5d02edc0-f3ec-11ec-b8ef-27feb720b1bb', 'course name1', '2021-03-04 00:46:26.134651+00', -- draft
        '2021-03-04 00:46:26.134651+00', 'en', null, 'draft test description', 0, 0, '{}'::jsonb),
       ('e1aa3b64-f3ec-11ec-b8ef-2f4e066962a2', 'course name2', '2021-03-04 00:46:26.134651+00', -- draft
        '2021-03-04 00:46:26.134651+00', 'en', null, 'draft test description', 0, 0, '{}'::jsonb),
       ('f9d0133a-f3ec-11ec-b8ef-3323861c2ef0', 'course name3', '2021-03-06 00:46:26.134651+00', -- draft
        '2021-03-06 00:46:26.134651+00', 'he', null, 'draft test description', 0, 0, '{}'::jsonb);


insert into course (id, creator_id, author_id, live_id, draft_id, published_at)
values ('3a6a3660-f3ec-11ec-b8ef-071747fa2a0d', '1f241e1b-b537-493f-a230-075cb16315be', '1f241e1b-b537-493f-a230-075cb16315be',
        '566ed564-f3ec-11ec-b8ef-93b44835e7e4', '5d02edc0-f3ec-11ec-b8ef-27feb720b1bb', '2022-06-24 18:35:33.636556+00'),
       ('c6b4e4b2-f3ec-11ec-b8ef-fb3d447b215e', '1f241e1b-b537-493f-a230-075cb16315be', '1f241e1b-b537-493f-a230-075cb16315be',
        'db817a2c-f3ec-11ec-b8ef-37027ca902ea', 'e1aa3b64-f3ec-11ec-b8ef-2f4e066962a2', '2022-06-23 18:35:33.636556+00'),
       ('ef0c4d42-f3ec-11ec-b8ef-af940a9cfba5', '7b96a41c-e406-11eb-8176-efd86dd7f444', '7b96a41c-e406-11eb-8176-efd86dd7f444',
        'f393863c-f3ec-11ec-b8ef-43660dc59560', 'f9d0133a-f3ec-11ec-b8ef-3323861c2ef0', '2022-06-22 18:35:33.636556+00');


insert into course_data_jig (course_data_id, jig_id)
values ('566ed564-f3ec-11ec-b8ef-93b44835e7e4', '0cc084bc-7c83-11eb-9f77-e3218dffb008'),
       ('db817a2c-f3ec-11ec-b8ef-37027ca902ea', '3a71522a-cd77-11eb-8dc1-af3e35f7c743'),
       ('5d02edc0-f3ec-11ec-b8ef-27feb720b1bb', '0cc084bc-7c83-11eb-9f77-e3218dffb008'),
       ('e1aa3b64-f3ec-11ec-b8ef-2f4e066962a2', '3a71522a-cd77-11eb-8dc1-af3e35f7c743'),
       ('566ed564-f3ec-11ec-b8ef-93b44835e7e4', '3a71522a-cd77-11eb-8dc1-af3e35f7c743'),
       ('5d02edc0-f3ec-11ec-b8ef-27feb720b1bb', '3a71522a-cd77-11eb-8dc1-af3e35f7c743');


insert into course_data_module (id, 
        course_data_id, index, kind, is_complete, contents, created_at)
values ('ce6e6eb0-0ea6-11ed-9785-2712225b9473', 
        '5d02edc0-f3ec-11ec-b8ef-27feb720b1bb', 0, 0, true, '{}', '2021-03-04 00:47:26.134651+00'), -- draft
       ('1cce16aa-0ea7-11ed-9785-0f0a17e25479', 
        'e1aa3b64-f3ec-11ec-b8ef-2f4e066962a2', 0, 0, true, '{}', '2021-03-04 00:47:26.134651+00'), -- draft 
       ('266cbba8-0ea7-11ed-9785-0b828c5e915c', 
        'f9d0133a-f3ec-11ec-b8ef-3323861c2ef0', 0, 0, true, '{}', '2021-03-04 00:47:26.134651+00'); -- draft


