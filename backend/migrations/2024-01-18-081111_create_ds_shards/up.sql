CREATE TABLE ds_shards (
    id SERIAL PRIMARY KEY,
    ds_id INTEGER NOT NULL REFERENCES datasets(id),
    uri VARCHAR(255) NOT NULL,
    num_samples INTEGER NOT NULL,
    verified BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

SELECT diesel_manage_updated_at('ds_shards');
