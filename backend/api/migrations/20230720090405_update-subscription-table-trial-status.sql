alter table subscription
    drop column if exists auto_renew,
    add column if not exists is_trial boolean default false not null
;
