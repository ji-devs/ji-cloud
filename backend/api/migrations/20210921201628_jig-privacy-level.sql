alter table jig
    add column if not exists privacy_level smallint not null default 1;
-- 1 is unlisted

update jig
set privacy_level = 0
where is_public is true;

alter table jig
    drop column if exists is_public;
