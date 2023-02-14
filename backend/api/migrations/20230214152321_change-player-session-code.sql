-- increase jig player session code numeric digits range
alter table jig_player_session alter column index type int using (index::int);

alter table jig_player_session_instance alter column session_index type int using (session_index::int);
