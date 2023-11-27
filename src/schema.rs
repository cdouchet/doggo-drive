// @generated automatically by Diesel CLI.

diesel::table! {
    doggo_files (id) {
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        id -> Uuid,
        local_path -> Text,
        net_path -> Text,
        file_name -> Nullable<Text>,
    }
}

diesel::table! {
    users (id) {
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        id -> Uuid,
        username -> Text,
        password -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    doggo_files,
    users,
);
