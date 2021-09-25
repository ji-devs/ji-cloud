insert into jig_data (id, display_name, created_at, updated_at, language, last_synced_at, description, theme,
                      audio_background, audio_feedback_negative, audio_feedback_positive, direction, display_score,
                      drag_assist, track_assessments)
values ('d4cad43c-1dd5-11ec-8426-83d4a42e3ac9', 'name', '2021-03-04 00:46:26.134651+00', -- live
        '2021-03-04 00:46:26.134651+00', 'en', '2021-03-04 00:46:26.134651+00', 'test description', 0, null,
        array [0, 1], array [0, 1, 2], 0, true, true, true),
       ('d4cad4c8-1dd5-11ec-8426-a37eda7ce03f', 'name', '2021-03-04 00:46:26.134651+00', -- draft
        '2021-03-04 00:46:26.134651+00', 'en', '2021-03-04 00:46:26.134651+00', 'test description', 0, null,
        array [0, 1], array [0, 1, 2], 0, true, true, true),
       ('d4cad52c-1dd5-11ec-8426-f7f3e8ceccb2', 'name', '2021-03-04 00:46:26.134651+00', -- live
        '2021-03-04 00:46:26.134651+00', 'en', '2021-03-04 00:46:26.134651+00', 'test description', 0, null,
        array [0, 1], array [0, 1, 2], 0, true, true, true),
       ('d4cad586-1dd5-11ec-8426-fbcd3fd01e2a', 'draft name', '2021-03-06 00:46:26.134651+00', -- draft
        '2021-03-06 00:46:26.134651+00', 'he', '2021-03-07 00:46:26.134651+00', 'draft test description', 1, 0,
        array []::smallint[], array []::smallint[], 1, false, false, false);


insert into jig (id, creator_id, author_id, privacy_level, live_id, draft_id)
values ('0cc084bc-7c83-11eb-9f77-e3218dffb008', '1f241e1b-b537-493f-a230-075cb16315be',
        '1f241e1b-b537-493f-a230-075cb16315be', 0, 'd4cad43c-1dd5-11ec-8426-83d4a42e3ac9',
        'd4cad4c8-1dd5-11ec-8426-a37eda7ce03f'),
       ('3a71522a-cd77-11eb-8dc1-af3e35f7c743', '1f241e1b-b537-493f-a230-075cb16315be',
        '1f241e1b-b537-493f-a230-075cb16315be', 0, 'd4cad52c-1dd5-11ec-8426-f7f3e8ceccb2',
        'd4cad586-1dd5-11ec-8426-fbcd3fd01e2a');

insert into jig_data_module (id, stable_id, jig_data_id, index, kind, is_complete, contents, created_at)
values ('a6b248f8-1dd7-11ec-8426-975953035335', '0cbfdd82-7c83-11eb-9f77-d7d86264c3bc',
        'd4cad43c-1dd5-11ec-8426-83d4a42e3ac9', 0, 0, false, '{}', '2021-03-04 00:46:26.134651+00'),
       ('a6b24970-1dd7-11ec-8426-57136b411853', '0cc03a02-7c83-11eb-9f77-f77f9ad65e9a',
        'd4cad43c-1dd5-11ec-8426-83d4a42e3ac9', 1, 1, false, '{}', '2021-03-04 00:46:26.134651+00'),
       ('a6b249c0-1dd7-11ec-8426-f77e58fd17c6', '5fac2632-cd4a-11eb-ae4e-7b9a797001a5',
        'd4cad43c-1dd5-11ec-8426-83d4a42e3ac9', 2, 3, false, '{}', '2021-03-04 00:46:26.134651+00'),
       ('a6b24a06-1dd7-11ec-8426-635a3a7ea572', '0cbfdd82-7c83-11eb-9f77-d7d86264c3bc',
        'd4cad4c8-1dd5-11ec-8426-a37eda7ce03f', 0, 0, false, '{}', '2021-03-04 00:46:26.134651+00'),
       ('a6b24a42-1dd7-11ec-8426-a7165f9281a2', '0cc03a02-7c83-11eb-9f77-f77f9ad65e9a',
        'd4cad4c8-1dd5-11ec-8426-a37eda7ce03f', 1, 1, false, '{}', '2021-03-04 00:46:26.134651+00'),
       ('a6b24a88-1dd7-11ec-8426-57525d09b22c', '5fac2632-cd4a-11eb-ae4e-7b9a797001a5',
        'd4cad4c8-1dd5-11ec-8426-a37eda7ce03f', 2, 3, false, '{}', '2021-03-04 00:46:26.134651+00');


insert into jig_data_additional_resource (jig_data_id, id, url)
values ('d4cad43c-1dd5-11ec-8426-83d4a42e3ac9', '286b828c-1dd9-11ec-8426-571b03b2d3df',
        'url://url.url.url/url'), -- live
       ('d4cad43c-1dd5-11ec-8426-83d4a42e3ac9', '286b82fa-1dd9-11ec-8426-2b6953011bae', 'url://test.url.testst/s/s'),
       ('d4cad4c8-1dd5-11ec-8426-a37eda7ce03f', '286b834a-1dd9-11ec-8426-6f641a50e23f',
        'url://url.url.url/url'), -- draft
       ('d4cad4c8-1dd5-11ec-8426-a37eda7ce03f', '286b8390-1dd9-11ec-8426-fbeb80c504d9', 'url://test.url.testst/s/s');

insert into jig_player_session (index, jig_id, created_at, expires_at, direction, display_score, track_assessments,
                                drag_assist)
values (1234, '0cc084bc-7c83-11eb-9f77-e3218dffb008', now(), now() - interval '5 minutes', 0, true, true, true),
       (1235, '0cc084bc-7c83-11eb-9f77-e3218dffb008', now(), now() + interval '5 minutes', 0, true, true, true);;

insert into jig_play_count (jig_id)
select id
from jig;
