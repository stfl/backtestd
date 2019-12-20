CREATE TABLE indicators (
    id        INTEGER GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    --  parent_id INTEGER REFERENCES indicators(id), -- a refined or updated indicator
    name      VARCHAR NOT NULL,
    value0 FLOAT4,
    range0 FLOAT4[],  -- {start, stop, step}  NULL if no range used
    value1 FLOAT4,
    range1 FLOAT4[],
    value2 FLOAT4,
    range2 FLOAT4[],
    value3 FLOAT4,
    range3 FLOAT4[],
    value4 FLOAT4,
    range4 FLOAT4[],
    value5 FLOAT4,
    range5 FLOAT4[],
    value6 FLOAT4,
    range6 FLOAT4[],
    value7 FLOAT4,
    range7 FLOAT4[],
    value8 FLOAT4,
    range8 FLOAT4[],
    value9 FLOAT4,
    range9 FLOAT4[],
    value10 FLOAT4,
    range10 FLOAT4[],
    value11 FLOAT4,
    range11 FLOAT4[],
    value12 FLOAT4,
    range12 FLOAT4[],
    value13 FLOAT4,
    range13 FLOAT4[],
    value14 FLOAT4,
    range14 FLOAT4[],
    value15 FLOAT4,
    range15 FLOAT4[],
    value16 FLOAT4,
    range16 FLOAT4[],
    value17 FLOAT4,
    range17 FLOAT4[],
    value18 FLOAT4,
    range18 FLOAT4[],
    value19 FLOAT4,
    range19 FLOAT4[],
    shift SMALLINT NOT NULL
);

/* CREATE TABLE indicator_samples (
 *     id INTEGER GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
 *     -- ref to the parent indicator with ranges
 *     indicator_id INTEGER NOT NULL REFERENCES indicators(id),
 *     value0 FLOAT4,
 *     value1 FLOAT4,
 *     value2 FLOAT4,
 *     value3 FLOAT4,
 *     value4 FLOAT4,
 *     value5 FLOAT4,
 *     value6 FLOAT4,
 *     value7 FLOAT4,
 *     value8 FLOAT4,
 *     value9 FLOAT4,
 *     value10 FLOAT4,
 *     value11 FLOAT4,
 *     value12 FLOAT4,
 *     value13 FLOAT4,
 *     value14 FLOAT4,
 *     value15 FLOAT4,
 *     value16 FLOAT4,
 *     value17 FLOAT4,
 *     value18 FLOAT4,
 *     value19 FLOAT4,
 *     shift SMALLINT NOT NULL
 * ); */

CREATE TABLE runs (
    id        INTEGER GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    result    FLOAT(4) NOT NULL,
    profit    FLOAT(4) NOT NULL,
    trades    INTEGER  NOT NULL,
    input_params JSONB
);

-- https://stackoverflow.com/questions/9789736/how-to-implement-a-many-to-many-relationship-in-postgresql
-- n:m relation between runs and indicator -> builds an indicator set
CREATE TABLE indicator_sets (
    run_id       INTEGER NOT NULL REFERENCES runs(id)       ON UPDATE CASCADE ON DELETE CASCADE,
    indicator_id INTEGER NOT NULL REFERENCES indicators(id) ON UPDATE CASCADE,
    indi_type    VARCHAR NOT NULL,                 -- TODO use an enum here
    CONSTRAINT set_id PRIMARY KEY (run_id, indicator_id)
);

CREATE TABLE samples (
    id     INTEGER GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    run_id INTEGER NOT NULL REFERENCES runs(id),
    params FLOAT4[],               -- can be NULL if no indi params are ranges
    result FLOAT(4) NOT NULL,
    profit FLOAT(4) NOT NULL,
    trades INTEGER  NOT NULL
);

/* CREATE TABLE indicator_sample_sets (
 *     sample_id      INTEGER NOT NULL REFERENCES samples(id)    ON UPDATE CASCADE ON DELETE CASCADE,
 *     indi_sample_id INTEGER NOT NULL REFERENCES indicator_samples(id) ON UPDATE CASCADE,
 *     indi_type      VARCHAR NOT NULL,                  -- TODO use an enum here
 *     CONSTRAINT sample_set_id PRIMARY KEY (sample_id, indi_sample_id)
 * ); */

/* CREATE TABLE run_params (
 *     id INTEGER GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
 *     run_id INTEGER REFERENCES runs(id) NOT NULL,
 *     name VARCHAR,            -- probably redundant
 *     date_from DATE NOT NULL,
 *     date_to DATE NOT NULL,
 *     backtest_model SMALLINT NOT NULL, -- TODO use an enum here
 *     optimize SMALLINT NOT NULL,        -- TODO use an enum here
 *     optimize_criteria SMALLINT NOT NULL,   -- TODO use an enum here
 *     visual BOOL NOT NULL,
 *     symbols TEXT[] NOT NULL
 * ); */
