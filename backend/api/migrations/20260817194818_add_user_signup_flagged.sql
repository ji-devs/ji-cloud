alter table "user" add column flagged boolean not null default false;
alter table "user" alter column flagged set default true;
create index user_flagged_idx on "user" (flagged) where flagged = true;
