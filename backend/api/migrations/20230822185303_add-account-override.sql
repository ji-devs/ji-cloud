alter table account
    add column if not exists tier_override smallint
;
