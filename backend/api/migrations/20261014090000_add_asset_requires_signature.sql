alter table jig_admin_data
    add column requires_signature boolean not null default true;

alter table playlist_admin_data
    add column requires_signature boolean not null default true;

alter table course_admin_data
    add column requires_signature boolean not null default true;
