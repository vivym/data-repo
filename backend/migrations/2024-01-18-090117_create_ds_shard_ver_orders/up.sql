CREATE TABLE ds_shard_ver_orders (
    id SERIAL PRIMARY KEY,
    ds_id INTEGER NOT NULL REFERENCES datasets(id),
    shard_id INTEGER NOT NULL REFERENCES ds_shards(id),
    sample_id INTEGER NOT NULL,
    score INTEGER NOT NULL CHECK(score BETWEEN 0 AND 5),
    comment TEXT NOT NULL DEFAULT '',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

SELECT diesel_manage_updated_at('ds_shard_ver_orders');