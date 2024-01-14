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
