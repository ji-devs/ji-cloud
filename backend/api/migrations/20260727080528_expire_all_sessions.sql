update session
set expires_at = now()
where expires_at is null or expires_at > now()
;
