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
    datasets_items_rel (ds_id, item_id) {
        ds_id -> Int4,
        item_id -> Int4,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    datasets_shards_rel (ds_id, shard_id) {
        ds_id -> Int4,
        shard_id -> Int4,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    ds_item_annos (id) {
        id -> Int4,
        item_id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        typ -> Varchar,
        #[max_length = 255]
        uri -> Nullable<Varchar>,
        number -> Nullable<Float8>,
        text -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    ds_items (id) {
        id -> Int4,
        #[max_length = 255]
        typ -> Varchar,
        #[max_length = 255]
        uri -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    ds_shards (id) {
        id -> Int4,
        #[max_length = 255]
        uri -> Varchar,
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
    groups_permissions_rel (group_id, permission_id) {
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
        is_active -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    users_groups_rel (user_id, group_id) {
        user_id -> Int4,
        group_id -> Int4,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(datasets_items_rel -> datasets (ds_id));
diesel::joinable!(datasets_items_rel -> ds_items (item_id));
diesel::joinable!(datasets_shards_rel -> datasets (ds_id));
diesel::joinable!(datasets_shards_rel -> ds_shards (shard_id));
diesel::joinable!(ds_item_annos -> ds_items (item_id));
diesel::joinable!(groups_permissions_rel -> groups (group_id));
diesel::joinable!(groups_permissions_rel -> permissions (permission_id));
diesel::joinable!(users_groups_rel -> groups (group_id));
diesel::joinable!(users_groups_rel -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    datasets,
    datasets_items_rel,
    datasets_shards_rel,
    ds_item_annos,
    ds_items,
    ds_shards,
    groups,
    groups_permissions_rel,
    permissions,
    users,
    users_groups_rel,
);
