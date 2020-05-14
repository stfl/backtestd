use super::schema::*;
use super::*;
use crate::signal_generator;

use crate::params;
use crate::params::legacy_indicator::LegacyIndicator;

use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;

use derive_more::Display;

// table! {
//     use diesel::sql_types::*;
//     use super::DbIndiFuncMapping;
//     indicator_default_func (indicator_id) {
//         indicator_id -> Int4,
//         func -> DbIndiFuncMapping,
//     }
// }

// joinable!(indicator_default_func -> indicators (indicator_id));

#[derive(Queryable, Associations, Identifiable, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[primary_key(indicator_id)]
#[table_name = "indicators"]
#[belongs_to(Indicator, foreign_key = "parent_id")]
// #[belongs_to(Indicator, foreign_key = "child_id") // FIXME but how?
pub struct Indicator {
    pub indicator_id: i32,
    pub parent_id: Option<i32>,
    pub child_id: Option<i32>,
    pub indicator_name: String,
    pub shift: i16,
    pub func: IndiFunc,
    pub class: Option<SignalClass>,
    pub filename: Option<String>,
    pub buffers: Option<Vec<i16>>,
    pub config: Option<Vec<BigDecimal>>,
}

#[derive(Insertable, Deserialize, Debug)]
#[table_name = "indicators"]
pub struct NewIndicator {
    pub parent_id: Option<i32>,
    pub child_id: Option<i32>,
    pub indicator_name: String, // TODO make borrowed?
    pub shift: i16,
    pub func: IndiFunc,
    pub class: Option<SignalClass>,
    pub filename: Option<String>, // TODO make borrowed?
    pub buffers: Option<Vec<i16>>,
    pub config: Option<Vec<BigDecimal>>,
}

#[derive(
    Queryable,
    Insertable,
    Identifiable,
    Associations,
    Debug,
    Clone,
    PartialEq,
    Serialize,
    Deserialize,
)]
#[primary_key(indicator_id, index)]
#[belongs_to(Indicator, foreign_key = "indicator_id")]
#[table_name = "indicator_inputs"]
pub struct IndicatorInput {
    pub indicator_id: i32, // 1:m
    pub index: i16,
    pub input: Option<BigDecimal>,
    pub start: Option<BigDecimal>,
    pub stop: Option<BigDecimal>,
    pub step: Option<BigDecimal>,
}

#[derive(DbEnum, Debug, PartialEq, Eq, Hash, Copy, Clone, Serialize, Deserialize, Display)]
pub enum IndiFunc {
    Confirm,
    Confirm2,
    Confirm3,
    Baseline,
    Volume,
    Continue,
    Exit,
}

// FIXME define values same as in MQL
#[derive(DbEnum, Debug, PartialEq, Eq, Hash, Copy, Clone, Serialize, Deserialize, Display)]
pub enum SignalClass {
    Preset = 0,
    ZeroLineCross,
    TwoLinesCross,
    TwoLinesTwoLevelsCross,
    TwoLevelsCross,
    PriceCross,
    PriceCrossInverted,
    Semaphore,
    TwoLinesColorChange,
    ColorChange,
    BothLinesTwoLevelsCross,
    BothLinesLevelCross,
    SaturationLevels,
    SaturationLines,
    BothLinesSaturationLevels,
    SlopeChange,
    TwoLinesSlopeChange,
}

// #[derive(Queryable, Insertable, Identifiable, Associations, Debug)]
// #[primary_key(indicator_id)]
// #[belongs_to(Indicator, foreign_key = "indicator_id")]
// #[table_name = "indicator_default_func"]
// pub struct IndicatorDefaultFunc {
//     pub indicator_id: i32,
//     func: DbIndiFunc,
// }

// should not be implemented like this
// impl<'a> From<&'a LegacyIndicator> for NewIndicator<'a> {
//     fn from(indi: &'a LegacyIndicator) -> Self {
//         NewIndicator {
//             parent_id: None,
//             child_id: None,
//             indicator_name: &indi.name,
//             shift: indi.shift as i16,
//             func: DbIndiFunc::Confirm, // the default of Confirm is set here which is an abstraction
//         }
//     }
// }

impl<'a> From<(IndiFunc, &'a LegacyIndicator)> for NewIndicator {
    fn from((func, indi): (IndiFunc, &'a LegacyIndicator)) -> Self {
        NewIndicator {
            parent_id: None,
            child_id: None,
            indicator_name: indi.name.clone(),
            shift: indi.shift as i16,
            func, //: func.to_owned(),

            // FIXME this needs to be taken from generate Indicator
            class: None,
            filename: None,
            buffers: None,
            config: None,
        }
    }
}

impl From<(IndiFunc, LegacyIndicator)> for NewIndicator {
    fn from((func, indi): (IndiFunc, LegacyIndicator)) -> Self {
        NewIndicator {
            parent_id: None,
            child_id: None,
            indicator_name: indi.name,
            shift: indi.shift as i16,
            func,

            // FIXME this needs to be taken from generate Indicator
            class: None,
            filename: None,
            buffers: None,
            config: None,
        }
    }
}

impl<'a> From<(IndiFunc, &'a signal_generator::SignalParams)> for NewIndicator {
    fn from((func, indi): (IndiFunc, &'a signal_generator::SignalParams)) -> Self {
        NewIndicator {
            parent_id: None,
            child_id: None,
            indicator_name: indi.name.clone(),
            shift: indi.shift as i16,
            func, //: func.to_owned(),
            class: Some(indi.indi_type),
            filename: Some(indi.name_indi.clone()),
            buffers: Some(indi.buffers.clone()),
            config: match (&indi.levels, &indi.colors) {
                (Some(l), None) => Some(l.clone()),
                (None, Some(c)) => Some(c.clone()),
                (None, None) => None,
                (Some(l), Some(c)) => {
                    l.clone().append(&mut c.clone());
                    Some(l.clone())
                }
            },
        }
    }
}

// impl From<Indicator> for NewIndicator {
//     fn from(indi: &Indicator) -> Self {
//         NewIndicator {
//             parent_id: None,
//             child_id: None,
//             indicator_name: indi.name.to_owned(),
//             shift: indi.shift as i16,
//             func: indi.func,
//         }
//     }
// }

impl From<(Indicator, Vec<IndicatorInput>)> for LegacyIndicator {
    fn from((indi, mut indi_inputs): (Indicator, Vec<IndicatorInput>)) -> Self {
        indi_inputs.sort_by_key(|v| v.index);
        LegacyIndicator {
            name: indi.indicator_name,
            shift: indi.shift as u8,
            inputs: indi_inputs
                .iter()
                .map(|v| {
                    // let vv = v.clone();
                    let mut input_vec = Vec::<BigDecimal>::new();
                    if let Some(inp) = &v.input {
                        input_vec.push(inp.to_owned());
                    }
                    if let (Some(sta), Some(sto), Some(ste)) =
                        (v.start.to_owned(), v.stop.to_owned(), v.step.to_owned())
                    {
                        input_vec.extend(vec![sta, sto, ste]);
                    }
                    // TODO should not panic!
                    assert!(input_vec.len() > 0 && input_vec.len() <= 4);
                    input_vec
                })
                .collect(),
        }
    }
}

impl Indicator {
    pub fn store_child(
        self: Self,
        conn: &PgConnection,
        indi: &LegacyIndicator,
    ) -> QueryResult<Indicator> {
        let child = store_indicator(conn, indi, Some(self.indicator_id), self.func)?;
        self.set_child(conn, &child)
    }

    pub fn new_child_no_ref(self: &Self, conn: &PgConnection) -> QueryResult<Indicator> {
        use crate::database::schema::indicators::dsl::*;
        diesel::insert_into(indicators)
            .values(NewIndicator {
                parent_id: Some(self.id().to_owned()),
                child_id: None,
                indicator_name: self.indicator_name.to_owned(),
                shift: self.shift as i16,
                func: self.func,
                class: self.class,
                filename: self.filename.clone(),
                buffers: self.buffers.clone(),
                config: self.config.clone(),
            })
            .get_result(conn)
    }

    pub fn new_child(self: Self, conn: &PgConnection) -> QueryResult<Indicator> {
        use crate::database::schema::indicators::dsl::*;
        let child = self.new_child_no_ref(conn)?;
        let _ = self.set_child(conn, &child)?;
        Ok(child)
    }

    pub fn set_child(self: Self, conn: &PgConnection, indi: &Indicator) -> QueryResult<Indicator> {
        unimplemented!()
    }

    pub fn set_parent() -> QueryResult<Indicator> {
        unimplemented!()
    }

    pub fn get_parent(
        self: Self,
        conn: &PgConnection,
        indi: &Indicator,
    ) -> QueryResult<Option<Indicator>> {
        match indi.parent_id {
            Some(p) => Indicator::try_load(conn, p).map(|i| Some(i)),
            None => Ok(None),
        }
    }

    pub fn try_load(conn: &PgConnection, indi_id: i32) -> QueryResult<Indicator> {
        use schema::indicators::dsl::*;
        indicators.find(indi_id).first::<Indicator>(conn)
    }
}

pub fn store_signal_params(
    conn: &PgConnection,
    indi: &signal_generator::SignalParams,
    parent: Option<i32>,
    indi_func: IndiFunc,
) -> Result<Indicator, diesel::result::Error> {
    use schema::indicator_inputs::dsl::*;
    use schema::indicators::dsl::*;

    let mut new_db_indi = NewIndicator::from((indi_func, indi));
    new_db_indi.parent_id = parent;

    if parent == None {
        // TODO check if an indicator with this name is already in the database
    }

    let new_indi: Indicator = diesel::insert_into(indicators)
        .values(new_db_indi)
        .get_result(conn)?;

    let indi_inputs: Vec<IndicatorInput> = indi
        .inputs
        .iter()
        .enumerate()
        .map(|(i, (t, v))| match v.len() {
            1 => IndicatorInput {
                indicator_id: new_indi.indicator_id,
                index: i as i16,
                input: Some(v[0].to_owned()),
                start: None,
                stop: None,
                step: None,
            },
            3 => IndicatorInput {
                indicator_id: new_indi.indicator_id,
                index: i as i16,
                input: None,
                start: Some(v[0].to_owned()),
                stop: Some(v[1].to_owned()),
                step: Some(v[2].to_owned()),
            },
            4 => IndicatorInput {
                indicator_id: new_indi.indicator_id,
                index: i as i16,
                input: Some(v[0].to_owned()),
                start: Some(v[1].to_owned()),
                stop: Some(v[2].to_owned()),
                step: Some(v[3].to_owned()),
            },
            _ => panic!("wrong number values on input"),
        })
        .collect();

    let indi_inputs: Vec<IndicatorInput> = diesel::insert_into(indicator_inputs)
        .values(&indi_inputs)
        .get_results(conn)?;
    // TODO info!()
    println!(
        "inserted indicator: {:?}\nwith inputs: {:?}",
        new_indi, indi_inputs
    );

    Ok(new_indi)
}

// TODO implment a trait ToDb which LegacyIndicator implements
pub fn store_indicator(
    conn: &PgConnection,
    indi: &LegacyIndicator,
    parent: Option<i32>,
    indi_func: IndiFunc,
) -> Result<Indicator, diesel::result::Error> {
    use schema::indicator_inputs::dsl::*;
    use schema::indicators::dsl::*;

    let mut new_db_indi = NewIndicator::from((indi_func, indi));
    new_db_indi.parent_id = parent;

    if parent == None {
        // TODO check if an indicator with this name is already in the database
    }

    let new_indi: Indicator = diesel::insert_into(indicators)
        .values(new_db_indi)
        .get_result(conn)?;

    let indi_inputs: Vec<IndicatorInput> = indi
        .inputs
        .iter()
        .enumerate()
        .map(|(i, v)| match v.len() {
            1 => IndicatorInput {
                indicator_id: new_indi.indicator_id,
                index: i as i16,
                input: Some(v[0].to_owned()),
                start: None,
                stop: None,
                step: None,
            },
            3 => IndicatorInput {
                indicator_id: new_indi.indicator_id,
                index: i as i16,
                input: None,
                start: Some(v[0].to_owned()),
                stop: Some(v[1].to_owned()),
                step: Some(v[2].to_owned()),
            },
            4 => IndicatorInput {
                indicator_id: new_indi.indicator_id,
                index: i as i16,
                input: Some(v[0].to_owned()),
                start: Some(v[1].to_owned()),
                stop: Some(v[2].to_owned()),
                step: Some(v[3].to_owned()),
            },
            _ => panic!("wrong number values on input"),
        })
        .collect();

    let indi_inputs: Vec<IndicatorInput> = diesel::insert_into(indicator_inputs)
        .values(&indi_inputs)
        .get_results(conn)?;
    // TODO info!()
    println!(
        "inserted indicator: {:?}\nwith inputs: {:?}",
        new_indi, indi_inputs
    );

    Ok(new_indi)
}

// trait ToDb {
//     fn to_db(conn: &PgConnection) -> Result<(), Error>;
// }

// pub fn store_indicators_with_default_func(conn: &PgConnection, indis: &Vec<(DbIndiFunc, LegacyIndicator)>) -> QueryResult<Vec<DbIndicator>> {
//     let mut db_indis : Vec<Indicator> = vec![];
//     for (f, i) in indis {
//         let db_indi = store_indicator(conn, &i, None)?;
//         let _ = store_indicator_default_func(conn, f, &db_indi);
//         db_indis.push(db_indi);
//     }
//     Ok(db_indis)
// }

// pub fn store_indicator_default_func(conn: &PgConnection, indi_func: &DbIndiFunc, indi: &DbIndicator) -> QueryResult<DbIndicatorDefaultFunc> {
//     use self::indicator_default_func::dsl::*;
//     diesel::insert_into(indicator_default_func)
//         .values(IndicatorDefaultFunc {
//             indicator_id: indi.indicator_id,
//             func: indi_func.to_owned(),
//         })
//         .get_result(conn)
// }

pub fn load_db_indicator(
    conn: &PgConnection,
    indi_id: i32,
) -> Result<Indicator, diesel::result::Error> {
    use schema::indicators::dsl::*;
    indicators.find(indi_id).first::<Indicator>(conn)
}

pub fn load_indicator(
    conn: &PgConnection,
    indi_id: i32,
) -> QueryResult<(Indicator, Vec<IndicatorInput>)> {
    use schema::indicator_inputs::dsl::*;

    let indi = Indicator::try_load(conn, indi_id)?;

    let indi_inputs = IndicatorInput::belonging_to(&indi).get_results::<IndicatorInput>(conn)?;

    // let indi_inputs = indicator_inputs
    //     .filter(indicator_id.eq(indi_id))
    //     .load::<IndicatorInput>(conn)?;

    Ok((indi, indi_inputs))
}

pub fn find_db_indicator(
    conn: &PgConnection,
    indi: LegacyIndicator,
) -> Result<Option<(Indicator, Vec<IndicatorInput>)>, diesel::result::Error> {
    // TODO this requires a join and then checking if all lines for the inputs match

    // SELECT .. from
    // WHERE indicators.name == indi.name AND indicators.shift == indi.shift
    // JOIN indicator_inputs
    // ON indicators.id == indicator_inpus.indicator_id
    // ORDER BY indicators.id and indicator_inputs.index
    unimplemented!();
}
