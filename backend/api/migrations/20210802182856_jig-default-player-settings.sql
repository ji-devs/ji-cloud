alter table jig
    add column track_assessments bool not null default false,
    add column drag_assist       bool not null default false;

select trigger_updated_at('jig');
