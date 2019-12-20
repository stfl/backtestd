table! {
    indicator_sets (run_id, indicator_id) {
        run_id -> Int4,
        indicator_id -> Int4,
        indi_type -> Varchar,
    }
}

table! {
    indicators (id) {
        id -> Int4,
        name -> Varchar,
        value0 -> Nullable<Float4>,
        range0 -> Nullable<Array<Float4>>,
        value1 -> Nullable<Float4>,
        range1 -> Nullable<Array<Float4>>,
        value2 -> Nullable<Float4>,
        range2 -> Nullable<Array<Float4>>,
        value3 -> Nullable<Float4>,
        range3 -> Nullable<Array<Float4>>,
        value4 -> Nullable<Float4>,
        range4 -> Nullable<Array<Float4>>,
        value5 -> Nullable<Float4>,
        range5 -> Nullable<Array<Float4>>,
        value6 -> Nullable<Float4>,
        range6 -> Nullable<Array<Float4>>,
        value7 -> Nullable<Float4>,
        range7 -> Nullable<Array<Float4>>,
        value8 -> Nullable<Float4>,
        range8 -> Nullable<Array<Float4>>,
        value9 -> Nullable<Float4>,
        range9 -> Nullable<Array<Float4>>,
        value10 -> Nullable<Float4>,
        range10 -> Nullable<Array<Float4>>,
        value11 -> Nullable<Float4>,
        range11 -> Nullable<Array<Float4>>,
        value12 -> Nullable<Float4>,
        range12 -> Nullable<Array<Float4>>,
        value13 -> Nullable<Float4>,
        range13 -> Nullable<Array<Float4>>,
        value14 -> Nullable<Float4>,
        range14 -> Nullable<Array<Float4>>,
        value15 -> Nullable<Float4>,
        range15 -> Nullable<Array<Float4>>,
        value16 -> Nullable<Float4>,
        range16 -> Nullable<Array<Float4>>,
        value17 -> Nullable<Float4>,
        range17 -> Nullable<Array<Float4>>,
        value18 -> Nullable<Float4>,
        range18 -> Nullable<Array<Float4>>,
        value19 -> Nullable<Float4>,
        range19 -> Nullable<Array<Float4>>,
        shift -> Int2,
    }
}

table! {
    runs (id) {
        id -> Int4,
        timestamp -> Timestamptz,
        result -> Float4,
        profit -> Float4,
        trades -> Int4,
        input_params -> Nullable<Jsonb>,
    }
}

table! {
    samples (id) {
        id -> Int4,
        run_id -> Int4,
        params -> Nullable<Array<Float4>>,
        result -> Float4,
        profit -> Float4,
        trades -> Int4,
    }
}

joinable!(indicator_sets -> indicators (indicator_id));
joinable!(indicator_sets -> runs (run_id));
joinable!(samples -> runs (run_id));

allow_tables_to_appear_in_same_query!(
    indicator_sets,
    indicators,
    runs,
    samples,
);
