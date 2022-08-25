insert into "user" (id, created_at) values ('1f241e1b-b537-493f-a230-075cb16315be', '2020-08-08T00:11:21Z'::timestamptz);

insert into user_email (user_id, email, created_at) values ('1f241e1b-b537-493f-a230-075cb16315be', 'test@test.test', '2020-08-08T00:11:21Z'::timestamptz);

insert into "user_profile" (user_id, username, created_at, given_name, family_name, languages_spoken,
                    opt_into_edu_resources, over_18, timezone, organization)
values ('1f241e1b-b537-493f-a230-075cb16315be', 'test',
        '2020-08-08T00:11:21Z'::timestamptz, 'Bobby', 'Tables', '{en_US}', true, true, 'US/Pacific-New', 'test org');

-- password is 'password1'
insert into "user_auth_basic" (user_id, email, password) values ('1f241e1b-b537-493f-a230-075cb16315be', 'test@test.test', '$argon2id$v=19$m=8192,t=16,p=1$3f60oO10WmwVJ9MIFf1f6w$CcjLqbHaDP7cJXAut6S9cmgGg6NL2Jsg++aIpdvmaBg');

-- 17 = 0x11 = 0b1_0001
-- GENERAL_API + DELETE_ACCOUNT
insert into "session" (user_id, token, scope_mask) values ('1f241e1b-b537-493f-a230-075cb16315be', 'Uv9rrKftNlHV0w2cbCHhf7wmtt5wQq8V', 17);


-- 8 is "Manage Own Jigs"
insert into "user_scope" (user_id, scope) values ('1f241e1b-b537-493f-a230-075cb16315be', 8);
