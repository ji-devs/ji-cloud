insert into jig (id, creator_id, author_id, created_at, language, description, privacy_level, is_draft,
                 audio_background,
                 audio_feedback_positive, audio_feedback_negative)
values ('0cc084bc-7c83-11eb-9f77-e3218dffb008', '1f241e1b-b537-493f-a230-075cb16315be',
        '1f241e1b-b537-493f-a230-075cb16315be', '2021-03-04 00:46:26.134651+00', 'en', 'test description', 0, false,
        null,
        array [0, 1], array [0, 1, 2]),
       ('3a71522a-cd77-11eb-8dc1-af3e35f7c743', '1f241e1b-b537-493f-a230-075cb16315be',
        '1f241e1b-b537-493f-a230-075cb16315be', '2021-03-04 00:46:26.134651+00', 'en', 'test description', 0, false, 1,
        array []::smallint[], array []::smallint[]);

insert into jig (id, creator_id, author_id, created_at, language, description, privacy_level, is_draft, publish_at)
values ('d52b9ff8-cd74-11eb-8dc1-b760927dc672', '1f241e1b-b537-493f-a230-075cb16315be', -- draft jig
        '1f241e1b-b537-493f-a230-075cb16315be', '2021-03-04 00:46:26.134651+00', 'en', 'test description', 1, false,
        '9999-03-04 00:46:26.134651+00'),
       ('bdc17474-d4a8-11eb-b8bc-0242ac130003', '1f241e1b-b537-493f-a230-075cb16315be',
        '1f241e1b-b537-493f-a230-075cb16315be', '2021-03-04 00:46:26.134651+00', 'en', 'test description', 0, false,
        '2021-03-04 00:46:26.134651+00');


insert into jig_module (jig_id, id, index, kind, contents, created_at, is_complete)
values ('0cc084bc-7c83-11eb-9f77-e3218dffb008', '0cbfdd82-7c83-11eb-9f77-d7d86264c3bc', 0, 0, '{}',
        '2021-03-04 00:46:26.134651+00', false),
       ('0cc084bc-7c83-11eb-9f77-e3218dffb008', '0cc03a02-7c83-11eb-9f77-f77f9ad65e9a', 1, 1, '{}',
        '2021-03-04 00:46:26.134651+00', false),
       ('0cc084bc-7c83-11eb-9f77-e3218dffb008', '5fac2632-cd4a-11eb-ae4e-7b9a797001a5', 2, 3, '{
         "content": null
       }', '2021-03-04 00:46:26.134651+00', false);

with draftless_jig as (
    select array(select id
                 from jig
                          left join jig_draft_join on jig.id = jig_draft_join.live_id
                 where draft_id is null) as arr
)
select make_drafts(draftless_jig.arr)
from draftless_jig;

insert into jig_additional_resource (jig_id, id, url)
values ('0cc084bc-7c83-11eb-9f77-e3218dffb008', '41b8d0b4-aaff-4942-88ba-1a32fecdbd23', 'url://url.url.url/url'),
       ('0cc084bc-7c83-11eb-9f77-e3218dffb008', '7a227007-b957-4601-bf15-87a6b86da672', 'url://test.url.testst/s/s');

insert into jig_player_session (index, jig_id, created_at, expires_at, direction, display_score, track_assessments,
                                drag_assist)
values (1234, '0cc084bc-7c83-11eb-9f77-e3218dffb008', now(), now() - interval '5 minutes', 0, true, true, true),
       (1235, '0cc084bc-7c83-11eb-9f77-e3218dffb008', now(), now() + interval '5 minutes', 0, true, true, true);;

insert into jig_play_count (jig_id)
select id
from jig;
