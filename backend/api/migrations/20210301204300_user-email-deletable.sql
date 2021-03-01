alter table user_email drop constraint user_email_user_id_fkey;
alter table user_email add constraint user_email_user_id_fkey foreign key (user_id) references "user"(id) on delete cascade;
