-- Add migration script here
alter table "user" alter column organization drop not null;  
alter table "user" add column geocode text;
