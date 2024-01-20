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

diesel::table! {
    groups (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    groups_permissions (group_id, permission_id) {
        group_id -> Int4,
        permission_id -> Int4,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    permissions (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        hashed_password -> Varchar,
        #[max_length = 255]
        nickname -> Varchar,
        #[max_length = 255]
        avatar_uri -> Varchar,
        verified -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    users_groups (user_id, group_id) {
        user_id -> Int4,
        group_id -> Int4,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(ds_shard_ver_orders -> datasets (ds_id));
diesel::joinable!(ds_shard_ver_orders -> ds_shards (shard_id));
diesel::joinable!(ds_shards -> datasets (ds_id));
diesel::joinable!(groups_permissions -> groups (group_id));
diesel::joinable!(groups_permissions -> permissions (permission_id));
diesel::joinable!(users_groups -> groups (group_id));
diesel::joinable!(users_groups -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    datasets,
    ds_shard_ver_orders,
    ds_shards,
    groups,
    groups_permissions,
    permissions,
    users,
    users_groups,
);
