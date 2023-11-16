drop trigger purge_expired_jig_player_session on jig_player_session;


alter table jig_player_session rename to jig_code;

alter table jig_code rename column index to code;


alter table jig_player_session_instance rename to jig_code_session;

alter table jig_code_session rename column session_index to code;
alter table jig_code_session rename column report to info;
UPDATE jig_code_session SET started_at = '2000-01-01 12:00:00.000000+00' where started_at is null;
alter table jig_code_session alter column started_at set not null;
