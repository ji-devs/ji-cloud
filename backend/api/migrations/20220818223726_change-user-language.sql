alter table user_profile
    alter language drop default,
    alter language type text[] using array[language],
    alter language set default '{}',
    add column    language_app         text         default 'en'             not null,
    add column    language_emails      text         default 'en'             not null,
    drop column   locale;

alter table user_profile
    rename column language_public to language_spoken_public;

alter table user_profile
      rename column language to language_spoken;

alter table jig_data 
      alter column language set default ''::text;

alter table resource_data 
      alter column language set default ''::text;

alter table course_data 
      alter column language set default ''::text;

update user_profile
set last_synced_at = null
where last_synced_at is not null;
