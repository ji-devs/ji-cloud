insert into "user" (id, created_at)
values ('1f241e1b-b537-493f-a230-075cb16315be', '2020-08-08T00:11:21Z'::timestamptz), -- user 0: complete admin, used for general API testing
       ('7b96a41c-e406-11eb-8176-efd86dd7f444', '2020-08-08T00:11:21Z'::timestamptz), -- user 1: incomplete user, needs profile to complete registration
       ('a641fd6e-e41b-11eb-8176-57df101c2201', now()); -- user 2: registering user, needs email verification

insert into user_email (user_id, email, created_at)
values ('1f241e1b-b537-493f-a230-075cb16315be', 'test@test.test', '2020-08-08T00:11:21Z'::timestamptz),
       ('7b96a41c-e406-11eb-8176-efd86dd7f444', 'fooodoosfakeemail@testing238EE.test', '2020-08-08T00:11:21Z'::timestamptz);

-- password is 'password1'
insert into "user_auth_basic" (user_id, email, password) values ('1f241e1b-b537-493f-a230-075cb16315be', 'test@test.test', '$argon2id$v=19$m=8192,t=16,p=1$3f60oO10WmwVJ9MIFf1f6w$CcjLqbHaDP7cJXAut6S9cmgGg6NL2Jsg++aIpdvmaBg'), -- 0
                                                                ('7b96a41c-e406-11eb-8176-efd86dd7f444', 'fooodoosfakeemail@testing238EE.test', '$argon2id$v=19$m=8192,t=16,p=1$3f60oO10WmwVJ9MIFf1f6w$CcjLqbHaDP7cJXAut6S9cmgGg6NL2Jsg++aIpdvmaBg'), -- 1
                                                                ('a641fd6e-e41b-11eb-8176-57df101c2201', 'fooodoosfakeemail23@teting28FE.test', '$argon2id$v=19$m=8192,t=16,p=1$3f60oO10WmwVJ9MIFf1f6w$CcjLqbHaDP7cJXAut6S9cmgGg6NL2Jsg++aIpdvmaBg'); -- 2

-- 17 = 0x11 = 0b1_0001
-- GENERAL_API + DELETE_ACCOUNT
insert into "session" (user_id, token, scope_mask) values ('1f241e1b-b537-493f-a230-075cb16315be', 'Uv9rrKftNlHV0w2cbCHhf7wmtt5wQq8V', 17), -- user 0
                                                          ('7b96a41c-e406-11eb-8176-efd86dd7f444', 'Sa84_qiKlh7WbOxeR9lofYJngysK_unF', 2), -- user 1
                                                          ('a641fd6e-e41b-11eb-8176-57df101c2201', 'L6gfXvgZeUBt8pdmLBnsGPEWUe3qGCK2_DF', 4); -- user 2

insert into "user_profile" (user_id, username, created_at, given_name, family_name, language, locale,
                    opt_into_edu_resources, over_18, timezone, organization)
values ('1f241e1b-b537-493f-a230-075cb16315be', 'test',
        '2020-08-08T00:11:21Z'::timestamptz, 'Bobby', 'Tables', 'en_US', 'en_US', true, true, 'US/Pacific', 'test org');

-- 1 is "Admin"
insert into "user_scope" (user_id, scope)
values ('1f241e1b-b537-493f-a230-075cb16315be', 1);
--