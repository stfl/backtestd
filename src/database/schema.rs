table! {
    expert_inputs (session_id, input_name) {
        session_id -> Int4,
        input_name -> Varchar,
        input -> Nullable<Numeric>,
        start -> Nullable<Numeric>,
        stop -> Nullable<Numeric>,
        step -> Nullable<Numeric>,
        str_input -> Nullable<Varchar>,
    }
}

table! {
    indicator_inputs (indicator_id, index) {
        indicator_id -> Int4,
        index -> Int2,
        input -> Nullable<Numeric>,
        start -> Nullable<Numeric>,
        stop -> Nullable<Numeric>,
        step -> Nullable<Numeric>,
    }
}

table! {
    indicator_inputs_explicit (inputs_id) {
        inputs_id -> Int8,
        indicator_id -> Int4,
        input0 -> Nullable<Numeric>,
        input1 -> Nullable<Numeric>,
        input2 -> Nullable<Numeric>,
        input3 -> Nullable<Numeric>,
        input4 -> Nullable<Numeric>,
        input5 -> Nullable<Numeric>,
        input6 -> Nullable<Numeric>,
        input7 -> Nullable<Numeric>,
        input8 -> Nullable<Numeric>,
        input9 -> Nullable<Numeric>,
        input10 -> Nullable<Numeric>,
        input11 -> Nullable<Numeric>,
        input12 -> Nullable<Numeric>,
        input13 -> Nullable<Numeric>,
        input14 -> Nullable<Numeric>,
    }
}

table! {
    indicator_sets (indicator_set_id) {
        indicator_set_id -> Int8,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::database::indicator::IndiFuncMapping;
    use crate::database::indicator::SignalClassMapping;
    indicators (indicator_id) {
        indicator_id -> Int4,
        parent_id -> Nullable<Int4>,
        child_id -> Nullable<Int4>,
        indicator_name -> Varchar,
        shift -> Int2,
        func -> IndiFuncMapping,
        class -> Nullable<SignalClassMapping>,
        filename -> Nullable<Varchar>,
        buffers -> Nullable<Array<Int2>>,
        config -> Nullable<Array<Numeric>>,
    }
}

table! {
    result_sets (result_id, inputs_id) {
        result_id -> Int8,
        inputs_id -> Int8,
    }
}

table! {
    results (result_id) {
        result_id -> Int8,
        run_id -> Int8,
        result -> Float4,
        profit -> Float4,
        expected_payoff -> Float4,
        profit_factor -> Float4,
        recovery_factor -> Float4,
        sharpe_ratio -> Float4,
        custom_result -> Float4,
        equity_drawdown -> Float4,
        trades -> Int4,
    }
}

table! {
    run_sessions (session_id) {
        session_id -> Int4,
        start_date -> Date,
        end_date -> Date,
        expert_version -> Nullable<Uuid>,
        symbol_set_id -> Int4,
    }
}

table! {
    runs (run_id) {
        run_id -> Int8,
        session_id -> Int4,
        run_date -> Timestamp,
        indicator_set_id -> Int8,
    }
}

table! {
    set_indicators (indicator_set_id, indicator_id) {
        indicator_set_id -> Int8,
        indicator_id -> Int4,
    }
}

table! {
    set_symbols (symbol_set_id) {
        symbol_set_id -> Int4,
        symbol -> Varchar,
    }
}

table! {
    symbol_sets (symbol_set_id) {
        symbol_set_id -> Int4,
    }
}

joinable!(expert_inputs -> run_sessions (session_id));
joinable!(indicator_inputs -> indicators (indicator_id));
joinable!(indicator_inputs_explicit -> indicators (indicator_id));
joinable!(result_sets -> indicator_inputs_explicit (inputs_id));
joinable!(result_sets -> results (result_id));
joinable!(results -> runs (run_id));
joinable!(run_sessions -> symbol_sets (symbol_set_id));
joinable!(runs -> indicator_sets (indicator_set_id));
joinable!(runs -> run_sessions (session_id));
joinable!(set_indicators -> indicator_sets (indicator_set_id));
joinable!(set_indicators -> indicators (indicator_id));
joinable!(set_symbols -> symbol_sets (symbol_set_id));

allow_tables_to_appear_in_same_query!(
    expert_inputs,
    indicator_inputs,
    indicator_inputs_explicit,
    indicator_sets,
    indicators,
    result_sets,
    results,
    run_sessions,
    runs,
    set_indicators,
    set_symbols,
    symbol_sets,
);
