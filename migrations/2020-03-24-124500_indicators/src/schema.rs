table! {
    indicator_names (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    indicators (id) {
        id -> Int4,
        parent_ranged_id -> Nullable<Int4>,
        idx -> Int2,
        input_id -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    indicator_names,
    indicators,
);
