insert into image_metadata (id, name, description, is_premium, created_at, kind)
values ('3095d05e-f2c7-11ea-89c3-3b621dd74a1f', 'test', 'testest', false, '2020-09-09T18:06:31.575087Z', 0),
       ('8cca6f3a-c4bb-11eb-8edf-13c75672da8f', 'test1', 'test description 1', false, '2021-05-01T18:06:31.575087Z', 0),
       ('8cca7124-c4bb-11eb-8edf-7b42383ed8f5', 'test2', 'test description 2', false, '2021-05-01T18:06:31.575087Z', 0),
       ('8cca719c-c4bb-11eb-8edf-f7accb638a15', 'test3', 'test description ', false, '2021-05-01T18:06:31.575087Z', 0),
       ('8cca720a-c4bb-11eb-8edf-63da1d86939c', 'test4', 'test description ', false, '2021-05-01T18:06:31.575087Z', 0);


insert into user_recent_image (user_id, image_id, media_library, last_used)
values ('1f241e1b-b537-493f-a230-075cb16315be', '8cca6f3a-c4bb-11eb-8edf-13c75672da8f', 1, '2021-06-03 22:30:48.451362'),
       ('1f241e1b-b537-493f-a230-075cb16315be', '8cca7124-c4bb-11eb-8edf-7b42383ed8f5', 1, '2021-06-03 22:30:45.451362'),
       ('1f241e1b-b537-493f-a230-075cb16315be', '8cca719c-c4bb-11eb-8edf-f7accb638a15', 1, '2021-06-03 22:30:46.451362'),
       ('1f241e1b-b537-493f-a230-075cb16315be', '8cca720a-c4bb-11eb-8edf-63da1d86939c', 1, '2021-06-03 22:30:47.451362');
