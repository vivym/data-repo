// @generated automatically by Diesel CLI.

diesel::table! {
    datasets (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        description -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    ds_shard_ver_orders (id) {
        id -> Int4,
        ds_id -> Int4,
        shard_id -> Int4,
        sample_id -> Int4,
        score -> Int4,
        comment -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    ds_shards (id) {
        id -> Int4,
        ds_id -> Int4,
        #[max_length = 255]
        uri -> Varchar,
        num_samples -> Int4,
        verified -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(ds_shard_ver_orders -> datasets (ds_id));
diesel::joinable!(ds_shard_ver_orders -> ds_shards (shard_id));
diesel::joinable!(ds_shards -> datasets (ds_id));

diesel::allow_tables_to_appear_in_same_query!(
    datasets,
    ds_shard_ver_orders,
    ds_shards,
);
