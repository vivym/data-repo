-- This file should undo anything in `up.sql`
DROP TABLE ds_shard_ver_orders;
DROP TRIGGER set_timestamp ON ds_shard_ver_orders;
