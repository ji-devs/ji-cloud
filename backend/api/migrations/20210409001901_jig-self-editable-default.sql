-- All users start out with the ability to create jigs for themselves.
insert into user_scope (user_id, scope)
select id as user_id, 8 as scope from "user";
