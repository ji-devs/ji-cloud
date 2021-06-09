-- Add migration script here
create table jig_draft_join
(
    draft_id uuid references jig (id) on delete cascade unique not null,
    live_id  uuid references jig (id) on delete cascade not null check (live_id != draft_id)
);
