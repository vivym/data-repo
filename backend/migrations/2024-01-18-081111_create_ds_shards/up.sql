CREATE TABLE ds_shards (
    id SERIAL PRIMARY KEY,
    uri VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

SELECT diesel_manage_updated_at('ds_shards');

CREATE TABLE datasets_shards_rel (
    ds_id INTEGER NOT NULL REFERENCES datasets(id),
    shard_id INTEGER NOT NULL REFERENCES ds_shards(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    PRIMARY KEY(ds_id, shard_id)
);
