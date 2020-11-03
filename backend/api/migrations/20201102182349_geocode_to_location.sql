-- Add migration script here
alter table "user"
    add column "location" jsonb;
