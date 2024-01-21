CREATE TABLE ds_items (
    id SERIAL PRIMARY KEY,
    typ VARCHAR(255) NOT NULL,
    uri VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

SELECT diesel_manage_updated_at('ds_items');

CREATE TABLE ds_item_annos (
    id SERIAL PRIMARY KEY,
    item_id INTEGER NOT NULL REFERENCES ds_items(id),
    name VARCHAR(255) NOT NULL,
    typ VARCHAR(255) NOT NULL,
    uri VARCHAR(255),
    number FLOAT,
    text TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

SELECT diesel_manage_updated_at('ds_item_annos');

CREATE TABLE datasets_items_rel (
    ds_id INTEGER NOT NULL REFERENCES datasets(id),
    item_id INTEGER NOT NULL REFERENCES ds_items(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    PRIMARY KEY(ds_id, item_id)
);
