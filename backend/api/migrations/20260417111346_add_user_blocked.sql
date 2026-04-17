alter table "user" add column blocked boolean not null default false;
create index user_blocked_idx on "user" (blocked) where blocked = true;
