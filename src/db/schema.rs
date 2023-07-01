// @generated automatically by Diesel CLI.

diesel::table! {
    lists (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    tasks (id) {
        id -> Nullable<Integer>,
        title -> Text,
        description -> Text,
        completed -> Integer,
        date -> Nullable<Text>,
        list_id -> Integer,
    }
}

diesel::joinable!(tasks -> lists (list_id));

diesel::allow_tables_to_appear_in_same_query!(
    lists,
    tasks,
);
