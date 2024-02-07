insert into course_data (id, display_name, created_at,
                updated_at, language, last_synced_at, description,  privacy_level, draft_or_live, translated_description, duration)
values ('f77221ca-906b-11ed-b4f6-0f507cdba0a1', 'course name1', '2021-03-04 00:46:26.134651+00', -- live
        '2021-03-04 00:46:26.134651+00', 'en', '2021-03-04 00:46:26.134651+00', 'live test description', 0, 1, '{}'::jsonb, null),
       ('47b3bf86-906c-11ed-b4f6-87f9f72a1171', 'course name2', '2021-03-04 00:46:26.134651+00', -- live
        '2021-03-04 00:46:26.134651+00', 'en', '2021-03-04 00:46:26.134651+00', 'live test description', 0, 1, '{}'::jsonb, null),
       ('be5a701c-906c-11ed-b4f6-8b8201faa1c7', 'course name3', '2021-03-04 00:46:26.134651+00', -- live
        '2021-03-04 00:46:26.134651+00', 'en', '2021-03-04 00:46:26.134651+00', 'live test description', 0, 1, '{}'::jsonb, null),
       ('f7722350-906b-11ed-b4f6-7f6f12b6b72a', 'course name1', '2021-03-04 00:46:26.134651+00', -- draft
        '2021-03-04 00:46:26.134651+00', 'en', null, 'draft test description', 0, 0, '{}'::jsonb, null),
       ('47b3c15c-906c-11ed-b4f6-abb1889c9b40', 'course name2', '2021-03-04 00:46:26.134651+00', -- draft
        '2021-03-04 00:46:26.134651+00', 'en', null, 'draft test description', 0, 0, '{}'::jsonb, null),
       ('be5a7120-906c-11ed-b4f6-7f0ad0c66ee8', 'course name3', '2021-03-06 00:46:26.134651+00', -- draft
        '2021-03-06 00:46:26.134651+00', 'he', null, 'draft test description', 0, 0, '{}'::jsonb, null);



insert into course (id, creator_id, author_id, live_id, draft_id, published_at)
values ('f77222a6-906b-11ed-b4f6-2f6dfab2ea0a', '1f241e1b-b537-493f-a230-075cb16315be', '1f241e1b-b537-493f-a230-075cb16315be',
        'f77221ca-906b-11ed-b4f6-0f507cdba0a1', 'f7722350-906b-11ed-b4f6-7f6f12b6b72a', '2022-06-24 18:35:33.636556+00'),
       ('47b3c062-906c-11ed-b4f6-9b0c5b1939a1', '1f241e1b-b537-493f-a230-075cb16315be', '1f241e1b-b537-493f-a230-075cb16315be',
        '47b3bf86-906c-11ed-b4f6-87f9f72a1171', '47b3c15c-906c-11ed-b4f6-abb1889c9b40', '2022-06-23 18:35:33.636556+00'),
       ('be5a6ee6-906c-11ed-b4f6-4788ec1806f1', '7b96a41c-e406-11eb-8176-efd86dd7f444', '7b96a41c-e406-11eb-8176-efd86dd7f444',
        'be5a701c-906c-11ed-b4f6-8b8201faa1c7', 'be5a7120-906c-11ed-b4f6-7f0ad0c66ee8', '2022-06-22 18:35:33.636556+00');

insert into course_data_unit (course_data_id, unit_id, display_name, description, created_at, updated_at, index, value)
values ('f7722350-906b-11ed-b4f6-7f6f12b6b72a', 'e0984370-906c-11ed-b4f6-3f864931e86f', 'course1 unit 1', 'Image', '2023-01-09 22:36:40.694823 +00:00', '2023-01-09 23:36:40.694823 +00:00', 0, '{"imageId": "3095d05e-f2c7-11ea-89c3-3b621dd74a1f"}'),
       ('47b3bf86-906c-11ed-b4f6-87f9f72a1171', '09225312-906d-11ed-b4f6-afb0e90115b3', 'course2 unit 1', 'PDF', '2023-01-09 22:36:40.694823 +00:00', '2023-01-09 23:36:40.694823 +00:00', 0, '{"pdfId": "128e0870-6813-11ec-a1fd-73f800abf897"}'),
    --    ('f7722350-906b-11ed-b4f6-7f6f12b6b72a', 'e0984460-906c-11ed-b4f6-4f0e8e23980c', "course1 unit 2", "Video", "2023-01-09 22:36:40.694823 +00:00", "2023-01-09 23:36:40.694823 +00:00", 1, '{video: {video}}'),
       ('47b3c15c-906c-11ed-b4f6-abb1889c9b40', '092253c6-906d-11ed-b4f6-87e87e8212af', 'course2 unit 2', 'Image', '2023-01-09 22:36:40.694823 +00:00', '2023-01-09 23:36:40.694823 +00:00', 1, '{"imageId": "92d8bb6e-0aa4-11ec-9f5e-83e2c58fc397"}'),
       ('f7722350-906b-11ed-b4f6-7f6f12b6b72a', 'e098451e-906c-11ed-b4f6-d3927726efad', 'course1 unit 2', 'Audio', '2023-01-09 22:36:40.694823 +00:00', '2023-01-09 23:36:40.694823 +00:00', 1, '{"audioId": "0d2c9c30-3850-11ed-ac6a-9b037f9e2276"}'),
       ('f7722350-906b-11ed-b4f6-7f6f12b6b72a', '09225240-906d-11ed-b4f6-ffc72cfa1509', 'course1 unit 3', 'URL', '2023-01-09 22:36:40.694823 +00:00', '2023-01-09 23:36:40.694823 +00:00', 2, '{"link": "https://jigzi.org/"}');


insert into course_data_module (id, stable_id,
        course_data_id, index, kind, is_complete, contents, created_at)
values ('3a272c30-906d-11ed-b4f6-1fef9f7412be', '4678e5cc-f057-478d-a7ed-044928c8ceb2',
        'f7722350-906b-11ed-b4f6-7f6f12b6b72a', 0, 0, true, '{}', '2021-03-04 00:47:26.134651+00'), -- draft
       ('3a272de8-906d-11ed-b4f6-8786ac4d1cf6', '4678e5cc-f057-478d-a7ed-044928c8ceb2',
        '47b3c15c-906c-11ed-b4f6-abb1889c9b40', 0, 0, true, '{}', '2021-03-04 00:47:26.134651+00'), -- draft
       ('3a272eec-906d-11ed-b4f6-cfd352276766', '4678e5cc-f057-478d-a7ed-044928c8ceb2',
        'be5a7120-906c-11ed-b4f6-7f0ad0c66ee8', 0, 0, true, '{}', '2021-03-04 00:47:26.134651+00'); -- draft
