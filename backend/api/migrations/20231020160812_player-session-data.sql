alter table jig_player_session_instance
    drop column user_agent,
    add column players_name text default null,
    add column started_at timestamptz default null,
    add column finished_at timestamptz default null,
    add column report json default null;
