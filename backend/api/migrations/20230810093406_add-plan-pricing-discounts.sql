alter table subscription
    add column if not exists price bigint,
    add column if not exists coupon_name text,
    add column if not exists coupon_percent decimal(12, 11),
    add column if not exists coupon_from timestamp with time zone,
    add column if not exists coupon_to timestamp with time zone
;

update subscription set price = 0 where price is null;

alter table subscription
    alter column price set not null
;
