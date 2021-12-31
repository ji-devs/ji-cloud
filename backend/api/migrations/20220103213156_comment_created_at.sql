alter table jig_curation_comment
    drop column created_at;

alter table jig_curation_comment
    add column created_at timestamptz   default now() not null;
    