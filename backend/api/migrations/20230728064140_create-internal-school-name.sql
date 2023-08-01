alter table school
    rename column school_name_id to internal_school_name_id
;

alter table school
    add column if not exists school_name citext,
    add column if not exists verified bool default false,
    alter column internal_school_name_id drop not null
;

alter table school_name
    drop column if exists verified
;

update school
    set school_name = school_name.name
from school_name
where
    school.internal_school_name_id = school_name.school_name_id
;

alter table school
    alter column school_name set not null
;
