-- This file should undo anything in `up.sql`
DROP TABLE datasets;
DROP TRIGGER set_timestamp ON datasets;
DROP FUNCTION trigger_set_timestamp();
