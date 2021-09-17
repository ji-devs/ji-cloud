alter table category 
    add column user_scopes smallint[] not null default '{}';
