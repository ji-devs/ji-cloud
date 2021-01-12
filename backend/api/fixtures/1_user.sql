insert into "user" (id, firebase_id, username, email, created_at, given_name, family_name, language, locale,
                    opt_into_edu_resources, over_18, timezone, organization)
values ('1f241e1b-b537-493f-a230-075cb16315be', 'SGkgdGhpcyBpcyBhIHRlc3QgdG9rZW4K', 'test', 'test@test.test',
        '2020-08-08T00:11:21Z'::timestamptz, 'Bobby', 'Tables', 'en_US', 'en_US', true, true, 'US/Pacific-New', 'test org');

-- 2 is "ManageCategory", 3 is "ManageImage", 4 is "ManageJig"
insert into "user_scope" (user_id, scope) values ('1f241e1b-b537-493f-a230-075cb16315be', 2), ('1f241e1b-b537-493f-a230-075cb16315be', 3), ('1f241e1b-b537-493f-a230-075cb16315be', 4), ('1f241e1b-b537-493f-a230-075cb16315be', 5), ('1f241e1b-b537-493f-a230-075cb16315be', 6);
