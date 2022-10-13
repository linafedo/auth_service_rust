-- Your SQL goes here
alter table tokens alter column created_at set default null;
alter table tokens alter column last_used_at set default null;