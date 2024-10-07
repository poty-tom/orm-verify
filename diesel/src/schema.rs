// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        complete -> Bool,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        age -> Int4,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    todos,
    users,
);