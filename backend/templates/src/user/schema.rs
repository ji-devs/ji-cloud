table! {
    roles (id) {
        id -> Int4,
        name -> Varchar,
        about -> Text,
    }
}

table! {
    users (id) {
        id -> Varchar,
        first_name -> Text,
        last_name -> Text,
        roles -> Array<Int4>,
        email -> Text,
        display_name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    roles,
    users,
);
