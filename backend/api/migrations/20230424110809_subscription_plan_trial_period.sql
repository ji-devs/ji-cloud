alter table subscription_plan
    add column trial_period int8 default 7 null;

-- Individual plans
update subscription_plan
    set trial_period = 7
where subscription_type = 0;

-- School plans
update subscription_plan
set trial_period = 14
where subscription_type = 1;
