use super::params::*;
use std::ffi::{OsStr, OsString};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use bigdecimal::BigDecimal;

use crate::database::indicator::SignalClass;

use serde_json::value::{self, Map, Value as Json};

use handlebars::{
    to_json, Context as HbContext, Handlebars, Helper, JsonRender, Output, RenderContext,
    RenderError,
};
// use serde_any;

use anyhow::{ensure, Context, Result};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct SignalParams {
    pub name: String,
    pub name_indi: String,
    pub indi_type: SignalClass,
    pub inputs: Vec<(InputType, Vec<BigDecimal>)>,
    pub buffers: Vec<i16>,
    pub levels: Option<Vec<BigDecimal>>, // up_enter, up_exit, down_enter, down_exit
    pub colors: Option<Vec<BigDecimal>>,  // COLOR_INDEX: neutr, up, down
    pub shift: u8,
}

// TODO impl TryFrom
impl From<&SignalParams> for Indicator {
    fn from(sig: &SignalParams) -> Self {
        Indicator {
            name: sig.name.clone(),
            inputs: sig
                .inputs
                .iter()
                .map(|i| match i.1.len() {
                    1 => i.1.clone(),      // only default value is given -> take it
                    4 => i.1[1..4].into(), // if range is given as well. take the range
                    _ => panic!("input length is invalid"),
                })
                .collect(),
            shift: sig.shift,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum InputType {
    Int = 0,
    Double = 1,
    String = 2,
}

// #[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
// pub enum SignalClass {
//     ZeroLineCross,
//     TwoLinesCross,
//     TwoLevelsCross,
//     PriceCross,
//     PriceCrossInverted,
//     Semaphore,
//     ColorChange,
// }

pub fn generate_signal(signal_params: &SignalParams, output_dir: &Path) -> Result<()> {
    match signal_params.indi_type {
        SignalClass::TwoLinesCross => {
            if signal_params.buffers.len() < 2
                || signal_params.buffers[0] == signal_params.buffers[1]
            {
                ensure!(false, "TwoLinesCross needs two different buffer indeces");
            }
        }
        SignalClass::TwoLevelsCross | SignalClass::ZeroLineCross => {
            ensure!(
                signal_params.buffers.len() == 1,
                "Only one buffer allowed for"
            );
        }
        _ => (),
    }
    let mut handlebars = Handlebars::new();

    fs::create_dir_all(output_dir)?;
    let mut output_file =
        File::create(output_dir.join(format!("Signal{}.mqh", signal_params.name)))
            .context("creating Signals header")?;

    let mut data = Map::new();
    data.insert("year".to_string(), to_json("2019"));
    data.insert("name".to_string(), to_json(&signal_params.name));
    data.insert("name_indi".to_string(), to_json(&signal_params.name_indi));
    data.insert("indi_type".to_string(), to_json(&signal_params.indi_type));
    data.insert(
        "inputs".to_string(),
        to_json::<Vec<Vec<String>>>(
            signal_params
                .inputs
                .iter()
                .map(|i| match i.0 {
                    InputType::Int => vec!["integer".to_string(), "INT".to_string()],
                    InputType::Double => vec!["double".to_string(), "DOUBLE".to_string()],
                    InputType::String => vec!["string".to_string(), "STRING".to_string()],
                })
                .collect(),
        ),
    );
    data.insert("buffers".to_string(), to_json(&signal_params.buffers));
    if signal_params.levels.is_some() {
        let levels = signal_params.levels.as_ref().unwrap();
        ensure!(levels.len() == 4, "wrong length of level inputs");
        data.insert("levels".to_string(), to_json(levels));
        // debug!("{:?}", data);
    }

    if signal_params.colors.is_some() {
        let colors = signal_params.colors.as_ref().unwrap();
        ensure!(colors.len() == 3, "wrong length of level inputs");
        data.insert("colors".to_string(), to_json(colors));
        // debug!("{:?}", data);
    }

    handlebars.register_helper("inc", Box::new(inc_helper));
    handlebars.register_helper("length", Box::new(length_helper));

    let tmpl_str = r#"//+------------------------------------------------------------------+
//|                                 Copyright {{year}}, Stefan Lendl |
//+------------------------------------------------------------------+
#include <..\Experts\BacktestExpert\Signal\\{{indi_type}}Signal.mqh>
#define PRODUCE_Signal{{name}} PRODUCE("{{name}}", CSignal{{name}})

class CSignal{{name}} : public C{{indi_type}}Signal {
public:
  CSignal{{name}}(void);
  virtual void      CSignal{{name}}::ParamsFromInput(double &Input[]);
};

CSignal{{name}}::CSignal{{name}}(void) {
  m_used_series=USE_SERIES_OPEN+USE_SERIES_HIGH+USE_SERIES_LOW+USE_SERIES_CLOSE;
  m_buf_idx = {{buffers.0}};
  {{#if buffers.1 ~}}
  m_down_idx = {{buffers.1}};
  {{/if ~}}
  {{#if levels ~}}
  m_level_up_enter   = {{levels.0}};
  m_level_up_exit    = {{levels.1}};
  m_level_down_enter = {{levels.2}};
  m_level_down_exit  = {{levels.3}};
  {{/if ~}}
  {{#if colors ~}}
  m_color_neutr = {{colors.0}};
  m_color_up    = {{colors.1}};
  m_color_down  = {{colors.2}};
  {{/if ~}}
}

void CSignal{{name}}::ParamsFromInput(double &Input[]) {
  m_params_size = {{length inputs}};
  ArrayResize(m_params, m_params_size);
  m_params[0].type=TYPE_STRING;
  m_params[0].string_value="Indi\\{{name_indi}}.ex5";
  {{#each inputs as |i| ~}}
  m_params[{{inc @index}}].type=TYPE_{{i.1}};
  m_params[{{inc @index}}].{{i.0}}_value=Input[{{@index}}];
  {{/each ~}}
}
"#;

    handlebars
        .render_template_to_write(tmpl_str, &data, &mut output_file)
        .context("writing template")?;
    Ok(())
}

fn inc_helper(
    h: &Helper,
    _: &Handlebars,
    _: &HbContext,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let val = h
        .param(0)
        .and_then(|v| v.value().as_u64())
        .ok_or(RenderError::new(
            "Param 0 with type u64 is required for inc helper",
        ))?;
    out.write(&(val + 1).to_string())?;
    Ok(())
}

fn length_helper(
    h: &Helper,
    _: &Handlebars,
    _: &HbContext,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let length = h
        .param(0)
        .as_ref()
        .and_then(|v| v.value().as_array())
        .map(|arr| arr.len())
        .ok_or(RenderError::new(
            "Param 0 with array type is required for rank helper",
        ))?;
    out.write(&(length + 1).to_string())?;
    Ok(())
}

pub fn generate_signal_includes(path: &PathBuf) -> Result<()> {
    // TODO this only checks for trailing /
    // ensure!(path.is_dir(), format!("{:?} is not a directory", path));

    if path.join("AllSignals.mqh").is_file() {
        fs::remove_file(path.join("AllSignals.mqh")).context("removing AllSignals.mqh")?;
    }
    let headers: Vec<OsString> = fs::read_dir(path)
        .context("reading signals header dir")?
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.path().file_name().map(|n| n.to_owned()) // map(|s| String::from(s)))
            })
        })
        .collect();
    debug!("generating AllSingnals.mqh for {:#?}", headers);
    let out = generate_includes(headers);
    // TSignalTestODO write out to AllSignals.mqh
    //
    let mut file = File::create(path.join("AllSignals.mqh")).context("create AllSignals.mqh")?;
    file.write_all(out.as_bytes())
        .context("writing AllSignals.mqh")?;
    Ok(())
}

fn generate_includes(headers: Vec<OsString>) -> String {
    let output: String = format!(
        "{includes}
#define PRODUCE_SIGNALS() \\
{producers}",
        includes = headers
            .iter()
            .map(|h| format!("#include \"{}\"\n", h.to_string_lossy()))
            .collect::<String>(),
        producers = headers
            .iter()
            .map(|h| {
                format!(
                    "PRODUCE_{} \\\n",
                    Path::new(h)
                        .file_stem()
                        .expect("something went wrong with the include filename")
                        .to_string_lossy()
                )
            })
            .collect::<String>()
    );
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_signal_test() {
        pretty_env_logger::init();

        let mut sig_params = SignalParams {
            name: "Test".to_string(),
            name_indi: "test".to_string(),
            indi_type: SignalClass::TwoLinesCross,
            inputs: vec![
                (InputType::Int, vec![0f32]),
                (InputType::Int, vec![0f32]),
                (InputType::Double, vec![0f32]),
                (InputType::Int, vec![0f32]),
            ],
            buffers: vec![0u8],
            levels: None,
            colors: None,
            shift: 0,
        };
        assert!(generate_signal(&sig_params, Path::new("/tmp")).is_err()); // only one buffer given
        sig_params.buffers = vec![0u8, 0u8]; // same buffer for TwoLineCross
        assert!(generate_signal(&sig_params, Path::new("/tmp")).is_err());
        sig_params.buffers[1] = 1u8;
        generate_signal(&sig_params, Path::new("/tmp")).unwrap();

        sig_params.indi_type = SignalClass::TwoLevelsCross;
        sig_params.buffers = vec![0u8];
        sig_params.levels = Some(vec![0f32]); // not enough levels
        assert!(generate_signal(&sig_params, Path::new("/tmp")).is_err());
        sig_params.levels = Some(vec![75f32, 60f32, 25f32, 40f32]);
        generate_signal(&sig_params, Path::new("/tmp")).unwrap();
        fs::remove_file("/tmp/SignalTest.mqh").unwrap();
    }

    #[test]
    fn generate_signal_include_test() {
        let headers: Vec<OsString> = vec![
            "asctrendsignal.mqh".into(),
            "pricechannel_stopsignal.mqh".into(),
            "SignalKijunSen.mqh".into(),
            "SignalWAE.mqh".into(),
            "supertrendsignal.mqh".into(),
        ];
        assert_eq!(
            generate_includes(headers),
            r#"#include "asctrendsignal.mqh"
#include "pricechannel_stopsignal.mqh"
#include "SignalKijunSen.mqh"
#include "SignalWAE.mqh"
#include "supertrendsignal.mqh"

#define PRODUCE_SIGNALS() \
PRODUCE_asctrendsignal \
PRODUCE_pricechannel_stopsignal \
PRODUCE_SignalKijunSen \
PRODUCE_SignalWAE \
PRODUCE_supertrendsignal \
"#
        );
    }

    /* #[test]
     * fn p_test() {
     *     generate_signal_includes(&PathBuf::from("/run/user/2000/gvfs/smb-share:server=192.168.122.22,share=metaquotes/Terminal/D0E8209F77C8CF37AD8BF550E51FF075/MQL5/Include/MyIndicators/Signals/")).unwrap();
     * } */

    #[test]
    fn from_signal_params_for_indicator_test() {
        let mut sig_params = SignalParams {
            name: "Test".to_string(),
            name_indi: "test".to_string(),
            indi_type: SignalClass::TwoLinesCross,
            inputs: vec![
                (InputType::Int, vec![1f32]),
                (InputType::Int, vec![10f32, 5f32, 20f32, 2f32]),
                (InputType::Double, vec![6.2]),
                (InputType::Double, vec![10f32, 6.1, 20f32, 0.5]),
            ],
            buffers: vec![0u8],
            levels: None,
            colors: None,
            shift: 0,
        };
        let indi = Indicator::from(&sig_params);
        assert_eq!(
            indi,
            Indicator {
                name: "Test".to_string(),
                inputs: vec![
                    vec![1f32],
                    vec![5f32, 20f32, 2f32],
                    vec![6.2],
                    vec![6.1, 20f32, 0.5],
                ],
                shift: 0,
            }
        );

        use serde_any;

        sig_params.inputs[1].1 = vec![10f32, 5f32, 20f32, 2f32, 3f32];
        let result = std::panic::catch_unwind(|| Indicator::from(&sig_params));
        assert!(result.is_err());
    }
}
