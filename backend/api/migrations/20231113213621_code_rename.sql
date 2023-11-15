drop trigger purge_expired_jig_player_session on jig_player_session;


alter table jig_player_session rename to jig_code;

alter table jig_code rename column index to code;


alter table jig_player_session_instance rename to jig_code_session;

alter table jig_code_session rename column session_index to code;
alter table jig_code_session rename column report to info;
alter table jig_code_session alter column started_at set not null;
