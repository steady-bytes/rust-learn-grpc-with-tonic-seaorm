table! {
    todo (id) {
        id -> Uuid,
        name -> Text,
        completed -> Nullable<Bool>,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        password -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    todo,
    users,
);
