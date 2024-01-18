-- This file should undo anything in `up.sql`
DROP TABLE ds_shards;
DROP TRIGGER set_timestamp ON ds_shards;
