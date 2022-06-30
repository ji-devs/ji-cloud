

insert into "circle" (id, display_name, description, creator_id, image)
values ('57a1eaaa-f182-11ec-a96e-13f3929f5b22', 'circle test', 'Test', '1f241e1b-b537-493f-a230-075cb16315be', '7100451a-0a9b-11ec-9f5e-17d67fa55c21'),
       ('829606d0-f185-11ec-b9e4-5fadfd7252f6', 'circle test1', 'Test1', '7b96a41c-e406-11eb-8176-efd86dd7f444', '71004740-0a9b-11ec-9f5e-ef9aad54a89e'),
       ('a3126bec-f185-11ec-b9e4-5fa4e257b5a1', 'circle test2', 'Test2', '1f241e1b-b537-493f-a230-075cb16315be', '92d8bd4e-0aa4-11ec-9f5e-93b65c5cdb5a');

insert into "circle_member" (id, user_id)
values ('829606d0-f185-11ec-b9e4-5fadfd7252f6', '1f241e1b-b537-493f-a230-075cb16315be'),
       ('829606d0-f185-11ec-b9e4-5fadfd7252f6', 'a641fd6e-e41b-11eb-8176-57df101c2201');