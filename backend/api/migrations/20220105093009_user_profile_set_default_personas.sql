alter table user_profile
    alter persona set default array[]::text[];

update user_profile set persona = '{}' where persona = '{NULL}' or persona is null;
