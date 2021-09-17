insert into category (id, parent_id, name, user_scopes, index, created_at)
values ('01cff7d8-e910-11ea-8281-7f86c625a156', 'e315d3b2-e90f-11ea-8281-73cd69c14821', 'A.1', '{1,4,3}', 0,
        '2020-08-28T09:22:34.737562Z'),
       ('afbce03c-e90f-11ea-8281-cfde02f6b582', null, 'Parent', '{1,6}', 0, '2020-08-28T09:20:17.039205Z'),
       ('e315d3b2-e90f-11ea-8281-73cd69c14821', 'afbce03c-e90f-11ea-8281-cfde02f6b582', 'A', '{}', 0,
        '2020-08-28T09:21:43.186002Z'),
       ('ee749392-e90f-11ea-8281-67d19ffa6107', 'afbce03c-e90f-11ea-8281-cfde02f6b582', 'B', '{6,7,8}', 1,
        '2020-08-28T09:22:02.261811Z');
