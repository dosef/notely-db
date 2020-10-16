table! {
    items (id) {
        id -> Uuid,
        list_id -> Uuid,
        title -> Varchar,
    }
}

table! {
    lists (id) {
        id -> Uuid,
        title -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    items,
    lists,
);
