    insert into style (id, display_name, index, created_at)
values ('6389eaa0-de76-11ea-b7ab-0399bcf84df2', 'A', 0, '2020-08-14T14:37:44.569428-07:00'),
       ('6389ff7c-de76-11ea-b7ab-9b5661dd4f70', 'B', 1, '2020-08-14T14:37:44.569428-07:00'),
       ('638a02a6-de76-11ea-b7ab-9300ed788cc1', 'C', 2, '2020-08-14T14:37:44.569428-07:00');

insert into affiliation (id, display_name, index, created_at)
values ('c0cd3d84-de76-11ea-b7ab-7fb0c04e8fe9', 'Apple Ichor', 0, '2020-08-14T14:40:21.040076-07:00'),
       ('c0cd4446-de76-11ea-b7ab-93987e8aa112', 'Brittle Brawns', 1, '2020-08-14T14:40:21.040076-07:00'),
       ('c0cd470c-de76-11ea-b7ab-c3aae340120b', 'Creative Creations', 2, '2020-08-14T14:40:21.040076-07:00');

insert into age_range (id, display_name, index, created_at)
values ('f37217d2-de76-11ea-b7ab-0b8ba2fb36ab', '0-12', 0, '2020-08-14T14:41:46.006699-07:00'),
       ('f372266e-de76-11ea-b7ab-3f6038429131', '13-17', 1, '2020-08-14T14:41:46.006699-07:00'),
       ('f3722790-de76-11ea-b7ab-77b45e9af3ef', '>= 18', 2, '2020-08-14T14:41:46.006699-07:00');

insert into subject (subject_id, display_name, index, created_at)
values ('5eac0740-f224-11ea-9b22-cf2ee94195fe', 'Subject A', 0, '2020-09-08T15:41:55.203383-07'),
       ('5eac07b8-f224-11ea-9b22-db3bdfab8258', 'Subject B', 1, '2020-09-08T15:41:55.203383-07'),
       ('5eac081c-f224-11ea-9b22-cbb8b4f00518', 'Subject C', 2, '2020-09-08T15:41:55.203383-07');

insert into resource_type (id, display_name, index, created_at)
values ('a91aca34-519e-11ec-ab46-175eaaf1ff23', 'Coloring', 0, '2020-09-08T15:41:55.203383-07'),
       ('a9219e7c-519e-11ec-ab46-63ad4c3d4eeb', 'Worksheet', 1, '2020-09-08T15:41:55.203383-07'),
       ('a939f454-519e-11ec-ab46-2fa68cd3a8c7', 'LessonPlan', 2, '2020-09-08T15:41:55.203383-07');

insert into "image_tag" (index, display_name, created_at)
values (0, 'A', '2021-04-22 19:53:37.997394+00'),
       (1, 'B', '2021-04-22 19:53:37.997394+00'),
       (2, 'C', '2021-04-22 19:53:37.997394+00');

insert into "animation_tag" (id, display_name, index, created_at)
values ('73f5daac-adec-11eb-ab06-5b9996c1d1a8', 'A', 0, '2021-05-05 21:45:56.665492+00'),
       ('73f5dd4a-adec-11eb-ab06-c39d2dbeb51f', 'B', 1, '2021-05-05 21:45:56.665492+00'),
       ('73f5df70-adec-11eb-ab06-97bd72e72a36', 'C', 2, '2021-05-05 21:45:56.665492+00'); 
