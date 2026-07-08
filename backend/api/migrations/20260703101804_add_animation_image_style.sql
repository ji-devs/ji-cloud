-- Add migration script here
insert into style (display_name, index)
values (
    'Animation',
    coalesce((select max(index) + 1 from style), 0)
);
