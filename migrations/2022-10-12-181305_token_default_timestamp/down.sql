-- This file should undo anything in `up.sql`
alter table tokens alter column created_at set default CURRENT_TIMESTAMP;
alter table tokens alter column last_used_at set default CURRENT_TIMESTAMP;