

insert into "badge" (id, display_name, description, creator_id, thumbnail)
values ('57a1eaaa-f182-11ec-a96e-13f3929f5b22', 'badge test', 'Test', '1f241e1b-b537-493f-a230-075cb16315be', 'https://www.jewishinteractive.org/wp-content/uploads/2021/12/customize.png'),
       ('829606d0-f185-11ec-b9e4-5fadfd7252f6', 'badge test1', 'Test1', '7b96a41c-e406-11eb-8176-efd86dd7f444', 'https://www.jewishinteractive.org/wp-content/uploads/2021/12/customize.png'),
       ('a3126bec-f185-11ec-b9e4-5fa4e257b5a1', 'badge test2', 'Test2', '1f241e1b-b537-493f-a230-075cb16315be', 'https://www.jewishinteractive.org/wp-content/uploads/2021/12/customize.png');

insert into "badge_member" (id, user_id)
values ('829606d0-f185-11ec-b9e4-5fadfd7252f6', '1f241e1b-b537-493f-a230-075cb16315be'),
       ('829606d0-f185-11ec-b9e4-5fadfd7252f6', 'a641fd6e-e41b-11eb-8176-57df101c2201');