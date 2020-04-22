-- This file should undo anything in `up.sql`
DROP TABLE set_indicators;
DROP TABLE indicator_sets;

DROP TABLE indicator_inputs_explicit;
DROP TABLE indicator_inputs;
-- DROP INDEX indicator_ranged_parent_index;
DROP INDEX parents_index;
DROP INDEX indi_names_index;
-- DROP TABLE indicator_default_func;
DROP TABLE indicators;
DROP TYPE SignalClass;
DROP TYPE IndiFunc;
