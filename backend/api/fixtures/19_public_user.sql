insert into "user_profile" (user_id, username, created_at, given_name, family_name, language, locale,
                    opt_into_edu_resources, over_18, timezone, organization, language_public, organization_public)
values ('7b96a41c-e406-11eb-8176-efd86dd7f444', 'test1',
        '2020-08-08T00:11:21Z'::timestamptz, 'Post', 'Gres', 'en_US', 'en_US', true, true, 'US/Pacific', 'test1 org', true, true),
        ('a641fd6e-e41b-11eb-8176-57df101c2201', 'test2',
        '2020-08-08T00:11:21Z'::timestamptz, 'Scrappy', 'Doo', 'en_US', 'en_US', true, true, 'US/Pacific', 'test2 org', false, true);

insert into user_follow(user_id, follower_id)
values ('7b96a41c-e406-11eb-8176-efd86dd7f444', 'a641fd6e-e41b-11eb-8176-57df101c2201'),
       ('a641fd6e-e41b-11eb-8176-57df101c2201', '7b96a41c-e406-11eb-8176-efd86dd7f444'),
       ('1f241e1b-b537-493f-a230-075cb16315be', 'a641fd6e-e41b-11eb-8176-57df101c2201'),
       ('1f241e1b-b537-493f-a230-075cb16315be', '7b96a41c-e406-11eb-8176-efd86dd7f444'),
       ('7b96a41c-e406-11eb-8176-efd86dd7f444', '1f241e1b-b537-493f-a230-075cb16315be'),
       ('a641fd6e-e41b-11eb-8176-57df101c2201', '1f241e1b-b537-493f-a230-075cb16315be');


