alter table settings 
    add column index_name text not null default '',
    add column index_hash text not null default '',
    add column updated_at timestamp with time zone,
    drop column "singleton",
    drop column algolia_index_version;

alter table settings
    rename to algolia_index_settings;

insert into algolia_index_settings (index_name)
values('media_index');

insert into algolia_index_settings (index_name)
values('jig_index');

insert into algolia_index_settings (index_name)
values('course_index');

