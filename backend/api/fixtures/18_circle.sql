

insert into "circle" (id, display_name, description, creator_id, image)
values ('57a1eaaa-f182-11ec-a96e-13f3929f5b22', 'circle test', 'Test', '1f241e1b-b537-493f-a230-075cb16315be', '89125d88-ffaa-11eb-86a5-9fd50ab8d8df'),
       ('829606d0-f185-11ec-b9e4-5fadfd7252f6', 'circle test1', 'Test1', '7b96a41c-e406-11eb-8176-efd86dd7f444', '89fa4c10-ffaa-11eb-86a5-870d6a01dc37'),
       ('a3126bec-f185-11ec-b9e4-5fa4e257b5a1', 'circle test2', 'Test2', '1f241e1b-b537-493f-a230-075cb16315be', '8a2469b4-ffaa-11eb-86a5-330f321d2a3f');

insert into "circle_member" (id, user_id)
values ('829606d0-f185-11ec-b9e4-5fadfd7252f6', '1f241e1b-b537-493f-a230-075cb16315be'),
       ('829606d0-f185-11ec-b9e4-5fadfd7252f6', 'a641fd6e-e41b-11eb-8176-57df101c2201'),
       ('a3126bec-f185-11ec-b9e4-5fa4e257b5a1', '7b96a41c-e406-11eb-8176-efd86dd7f444'),
       ('a3126bec-f185-11ec-b9e4-5fa4e257b5a1', 'a641fd6e-e41b-11eb-8176-57df101c2201'),
       ('57a1eaaa-f182-11ec-a96e-13f3929f5b22', '7b96a41c-e406-11eb-8176-efd86dd7f444');
