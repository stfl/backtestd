#[allow(dead_code)]

// use std::vec:Vec;
use std::path::{Path,PathBuf};
use std::ffi::OsStr;
use std::io;

const FOREX_PAIRS : &'static [&'static str] = &[
        "EURUSD", "GBPUSD", "USDCHF", "USDJPY", "USDCAD", "AUDUSD", "EURCHF",
        "EURJPY", "EURGBP", "EURCAD", "GBPCHF", "GBPJPY", "AUDJPY", "AUDNZD",
        "AUDCAD", "AUDCHF", "CHFJPY", "EURAUD", "EURNZD", "CADCHF", "GBPAUD",
        "GBPCAD", "GBPNZD", "NZDCAD", "NZDCHF", "NZDJPY", "NZDUSD", "CADJPY"];

#[derive(Debug, Default, PartialEq)]
pub struct Indicator {
    name : String,
    indi_type : String,
    inputs : Vec<Input>,
    shift : Option<u8>,
}

impl Indicator {
    // maybe implement io::Write instead?
    pub fn to_params_config(&self) -> String {
        let mut string : String = format!("{indi_type}_Indicator={name}\n",
                                  indi_type = self.indi_type,
                                  name = self.name);
        for (i, inp) in self.inputs.iter().enumerate() {
            string.push_str(&format!("{indi_type}_{input_type}{idx}=\
                                    {input_value}\n",
                                  indi_type = self.indi_type,
                                  input_type = inp.type_str(),
                                  input_value = inp.value_str(),
                                  idx = i,
                                  ));
        }
        match self.shift {
            Some(a) => string.push_str(&format!("{}_Shift={}\n", self.indi_type, a)),
            _ => {},
        }

        string
    }
}

#[derive(Debug, PartialEq)]
pub enum Input {
    Int(i32),
    Double(f32),
    IntRange((i32, i32, i32)),
    DoubleRange((f32, f32, f32)),
}

impl Input {
    fn type_str(&self) -> &str {
        match self {
            Input::Int(_) | Input::IntRange(_) => "Int",
            Input::Double(_) | Input::DoubleRange(_) => "Double",
        }
    }

    fn value_str(&self) -> String {
        match self {
            Input::Int(a) => format!("{}||0||0||0||N", a),
            Input::Double(a) => format!("{:.2}||0||0||0||N", a),
            Input::IntRange(a) => format!("0||{}||{}||{}||Y", a.0, a.1, a.2),
            Input::DoubleRange(a) => format!("0||{:.2}||{:.2}||{:.2}||Y", a.0, a.1, a.2),
        }
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct IndicatorSet {
    confirm : Option<Indicator>,
    confirm2 : Option<Indicator>,
    confirm3 : Option<Indicator>,
    exit : Option<Indicator>,
    cont : Option<Indicator>,
    baseline : Option<Indicator>,
    volume : Option<Indicator>,
}

impl IndicatorSet {
    fn to_params_config(&self) -> String {
        let mut string = String::new();
        match &self.confirm {
            Some(i) => string.push_str(&i.to_params_config()),
            _ => {},
        }
        match &self.confirm2 {
            Some(i) => string.push_str(&i.to_params_config()),
            _ => {},
        }
        match &self.confirm3 {
            Some(i) => string.push_str(&i.to_params_config()),
            _ => {},
        }
        match &self.cont {
            Some(i) => string.push_str(&i.to_params_config()),
            _ => {},
        }
        match &self.exit {
            Some(i) => string.push_str(&i.to_params_config()),
            _ => {},
        }
        match &self.baseline {
            Some(i) => string.push_str(&i.to_params_config()),
            _ => {},
        }
        match &self.volume {
            Some(i) => string.push_str(&i.to_params_config()),
            _ => {},
        }

        string
    }
}


// input from the API
#[derive(Default, Debug, PartialEq)]
pub struct RunParams {
    name : String,
    indi_set : IndicatorSet,
    date : (String, String),
    backtest_model : BacktestModel,
    optimize : OptimizeMode,
    optimize_crit :  OptimizeCrit,
    visual : bool,
    // symbols : &[],
    symbols : Vec<String>,
}

impl RunParams {
    pub fn to_params_config(&self) -> String {
        return self.indi_set.to_params_config();
    }

    fn to_config(&self) -> String {
        format!("
Visual={visual}
FromDate={from_date}
ToDate={to_date}
Model={model}
Optimization={opti}
OptimizationCriterion={opti_crit}",
                visual = self.visual as i32,
                from_date = self.date.0,
                to_date = self.date.1,
                model = self.backtest_model as u8,
                opti = self.optimize as u8,
                opti_crit = self.optimize_crit as u8)
    }

    pub fn new() -> Self {
        RunParams{
            name : "backtest".to_string(),
            indi_set : IndicatorSet::default(),
            date : ("2017.08.01".to_string(), "2019.08.20".to_string()),
            backtest_model : BacktestModel::default(),
            optimize : OptimizeMode::default(),
            optimize_crit :  OptimizeCrit::default(),
            visual : false,
            symbols : FOREX_PAIRS.iter().map(|s| s.to_string()).collect(),
                // to_vec().to_string(),
            // symbols_iter : symbols.iter()
        }
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.symbols.iter()
    }
}



/* impl Iterator for RunParams {
 *     type Item = String;
 *
 *     fn next(&mut self) -> Option<String> {
 *         let symbols_iter = self.symbols.iter();
 *         symbols_iter.next()
 *     }
 * } */



// terminal execution specific configuration
#[derive(Default, Debug, PartialEq)]
pub struct CommonParams {
    pub params_file : String,
    pub terminal_exe : PathBuf,
    pub workdir : PathBuf,
    pub reports : PathBuf,
    pub expert : String,
    pub period : String,
    pub login : String,
    pub use_remote : bool,
    pub use_local : bool,
    pub replace_report : bool,
    pub shutdown_terminal : bool,
    pub deposit : u32,
    pub currency : String,
    pub leverage : u16,
    pub execution_mode : u8,
    // run_params : RunParams,
}

impl CommonParams {
    pub fn new(workdir: &Path) -> Self {
        CommonParams {
            params_file : "expert_params.set".to_string(),
            terminal_exe : PathBuf::from(r"C:/Program Files/MetaTrader 5/terminal64.exe"),
            workdir : workdir.to_path_buf(),
            reports : workdir.join("reports"),
            // expert : "nnfx-ea/nnfx-ea.ex5".to_string(),
            expert : "expert/expert.ex5".to_string(),
            period : "D1".to_string(),
            login : "".to_string(),
            use_remote : true,
            use_local : true,
            replace_report : true,
            shutdown_terminal : true,
            deposit : 10000,
            currency : "USD".to_string(),
            leverage : 100,
            execution_mode : 0,
            // run_params : run,
        }
    }

    pub fn params_path(&self) -> PathBuf {
        let mut params_path = self.workdir.clone();
        params_path.push("MQL5/Profiles/Tester");
        params_path.push(&self.params_file);
        params_path
    }

    pub fn to_config(&self) -> String {
        format!("
Expert={expert}
ExpertParameters={params_file}
Period={period}
Login={login}
UseLocal={use_local}
UseRemote={use_remote}
ReplaceReport={replace_report}
ShutdownTerminal={shutdown_terminal}
Deposit={deposit}
Currency={currency}
Leverage={leverage}
ExecutionMode={exec_mode}",
                expert = self.expert,
                params_file = self.params_file,
                period = self.period,
                login = self.login,
                use_local = self.use_local as i32,
                use_remote = self.use_remote as i32,
                replace_report = self.replace_report as i32,
                shutdown_terminal = self.shutdown_terminal as i32,
                deposit = self.deposit,
                currency = self.currency,
                leverage = self.leverage,
                exec_mode = self.execution_mode)
    }
}

pub fn to_terminal_config(common : &CommonParams, run : &RunParams, symbol : &String) -> String {
        let mut reports_path = reports_dir(common, run).join(symbol);
        reports_path.set_extension("xml");
        let reports_path = reports_path.as_os_str();
        format!("[Tester]
{common}
{run}
Symbol={symb}
Report={report}",
                common = common.to_config(),
                run = run.to_config(),
                symb = symbol,
                report = reports_path.to_string_lossy()
                )
}

fn reports_dir(common : &CommonParams, run: &RunParams) -> PathBuf {
    common.reports.join(&run.name)
}


/* Expert={expert}
 * ExpertParameters={params_file}
 * Period={period}
 * Login={login}
 * Visual={visual}
 * UseLocal={use_local}
 * UseRemote={use_remote}
 * FromDate={from_date}
 * ToDate={to_date}
 * ReplaceReport={replace_report}
 * ShutdownTerminal={shutdown_terminal}
 * Deposit={deposit}
 * Currency={currency}
 * Leverage={leverage}
 * Model={model}
 * ExecutionMode={exec_mode}
 * Optimization={opti}
 * OptimizationCriterion={opti_crit}
 * ",
 * expert = terminal_params.expert,
 * params_file = terminal_params.params_file,
 * // symbol = run_params.next(),
 * period = terminal_params.period,
 * login = terminal_params.login,
 * visual = run_params.visual as i32,
 * use_local = terminal_params.use_local as i32,
 * use_remote = terminal_params.use_remote as i32,
 * from_date = run_params.date.0,
 * to_date = run_params.date.1,
 * replace_report = terminal_params.replace_report as i32,
 * shutdown_terminal = terminal_params.shutdown_terminal as i32,
 * deposit = terminal_params.deposit,
 * currency = terminal_params.currency,
 * leverage = terminal_params.leverage,
 * model = run_params.backtest_model as u8,
 * exec_mode = terminal_params.execution_mode,
 * opti = run_params.optimize as u8,
 * opti_crit = run_params.optimize_crit as u8,
 * ) */
// }

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum BacktestModel {
    EveryTick = 0, // "Every tick"
    OneMinuteOHLC = 1, // "1 minute OHLC"
    OpenPrice = 2, // "Open price only"
    MathCalc = 3, // "Math calculations"
    EveryTickReal = 4, // "Every tick based on real ticks"
}

// fn f(m: &)

impl Default for BacktestModel {
    fn default() -> Self { BacktestModel::OpenPrice }
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum OptimizeMode {
    Disabled = 0,  // optimization disabled
    Complete = 1,  // "Slow complete algorithm"
    Genetic = 2,  // "Fast genetic based algorithm"
    AllSymbols = 3,  // "All symbols selected in Market Watch"
}

impl Default for OptimizeMode {
    fn default() -> Self { OptimizeMode::Complete }
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum OptimizeCrit {
    Balance = 0,  // the maximum balance value, 
    BalanceProf = 1,  // the maximum value of product of the balance and profitability, 
    BalancePayoff = 2,  // the product of the balance and expected payoff, 
    Drawdown = 3,  // the maximum value of the expression (100% - Drawdown)*Balance, 
    BalanceRecovery = 4,  // the product of the balance and the recovery factor, 
    BalanceSharpe = 5,  // the product of the balance and the Sharpe Ratio, 
    Custom = 6,  // a custom optimization criterion received from the OnTester() function in the Expert Advisor).
}

impl Default for OptimizeCrit {
    fn default() -> Self { OptimizeCrit::Custom }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::Path;

    // #[test]
    // fn write_config_test() {
    //     let mut terminal_params : CommonParams = Default::default();
    //     assert_eq!(terminal_params,
    //                CommonParams{
    //                    params_file : "expert_params.set".to_string(),
    //                    terminal_exe : PathBuf::new(),
    //                    workdir : PathBuf::new(),
    //                    reports : "".to_string(),
    //                    expert : "".to_string(),
    //                    period : "".to_string(),
    //                    login : "".to_string(),
    //                    use_remote : false,
    //                    use_local : false,
    //                    replace_report : false,
    //                    shutdown_terminal : false,
    //                    deposit : 0,
    //                    currency : "".to_string(),
    //                    leverage : 0,
    //                    execution_mode : 0,
    //                });
    // }

     #[test]
     fn input_str_test() {
        let mut inp = Input::Int(0);
        assert_eq!(inp.type_str(), "Int");
        assert_eq!(inp.value_str(), "0||0||0||0||N");

        inp = Input::Double(0.);
        assert_eq!(inp.type_str(), "Double");
        assert_eq!(inp.value_str(), "0.00||0||0||0||N");

        inp = Input::IntRange((10,20,1));
        assert_eq!(inp.type_str(), "Int");
        assert_eq!(inp.value_str(), "0||10||20||1||Y");

        inp = Input::DoubleRange((10.,20.,1.));
        assert_eq!(inp.type_str(), "Double");
        assert_eq!(inp.value_str(), "0||10.00||20.00||1.00||Y");
     }

     #[test]
     fn indicator_config_test() {
         let mut indi = Indicator{
             name : "ama".to_string(),
             indi_type : "Confirm".to_string(),
             shift : None,
             inputs : Vec::new(),
         };
        assert_eq!(indi.to_params_config(), "Confirm_Indicator=ama\n");
        
        indi.shift = Some(7);
        assert_eq!(indi.to_params_config(),
"Confirm_Indicator=ama
Confirm_Shift=7
");

        indi.inputs.push(Input::Int(3));
        assert_eq!(indi.to_params_config(),
"Confirm_Indicator=ama
Confirm_Int0=3||0||0||0||N
Confirm_Shift=7
");

        indi.inputs.push(Input::Int(4));
        assert_eq!(indi.to_params_config(),
"Confirm_Indicator=ama
Confirm_Int0=3||0||0||0||N
Confirm_Int1=4||0||0||0||N
Confirm_Shift=7
");

        indi.inputs.push(Input::Double(5.));
        assert_eq!(indi.to_params_config(),
"Confirm_Indicator=ama
Confirm_Int0=3||0||0||0||N
Confirm_Int1=4||0||0||0||N
Confirm_Double2=5.00||0||0||0||N
Confirm_Shift=7
");

        indi.inputs.push(Input::IntRange((10,200,5)));
        assert_eq!(indi.to_params_config(),
"Confirm_Indicator=ama
Confirm_Int0=3||0||0||0||N
Confirm_Int1=4||0||0||0||N
Confirm_Double2=5.00||0||0||0||N
Confirm_Int3=0||10||200||5||Y
Confirm_Shift=7
");

        indi.inputs.push(Input::DoubleRange((10.,200.,0.5)));
        assert_eq!(indi.to_params_config(),
"Confirm_Indicator=ama
Confirm_Int0=3||0||0||0||N
Confirm_Int1=4||0||0||0||N
Confirm_Double2=5.00||0||0||0||N
Confirm_Int3=0||10||200||5||Y
Confirm_Double4=0||10.00||200.00||0.50||Y
Confirm_Shift=7
");
     }

     #[test]
     fn indi_set_config_test() {
         let mut indi_set = IndicatorSet{
             confirm : Some(Indicator{
                 name : "ama".to_string(),
                 inputs : vec![Input::Int(3),
                               Input::DoubleRange((10.,200.,0.5))],
                 indi_type : "Confirm".to_string(),
                 .. Default::default()
             }),
             ..Default::default()
         };
         assert_eq!(indi_set.to_params_config(),
"Confirm_Indicator=ama
Confirm_Int0=3||0||0||0||N
Confirm_Double1=0||10.00||200.00||0.50||Y
");

         indi_set.baseline = Some(Indicator{
                 name : "bama".to_string(),
                 inputs : vec![Input::Double(3.),
                               Input::DoubleRange((10.,200.,0.5))],
                 indi_type : "Baseline".to_string(),
                 .. Default::default()
             });
         assert_eq!(indi_set.to_params_config(),
"Confirm_Indicator=ama
Confirm_Int0=3||0||0||0||N
Confirm_Double1=0||10.00||200.00||0.50||Y
Baseline_Indicator=bama
Baseline_Double0=3.00||0||0||0||N
Baseline_Double1=0||10.00||200.00||0.50||Y
");
     }


     #[test]
     fn terminal_config_params_path_test() {
         let term_params = CommonParams {
             workdir : PathBuf::from(r"C:/workdir"),
             params_file : "test.set".to_string(),
             .. Default::default()
         };
         assert_eq!(term_params.params_path().as_path(),
                    Path::new("C:/workdir/MQL5/Profiles/Tester/test.set")
         );

         let term_params = CommonParams::new(
             Path::new("C:/Users/stele/AppData/Roaming/MetaQuotes/Terminal/D0E8209F77C8CF37AD8BF550E51FF075"));
         assert_eq!(term_params.params_path().as_path(),
                    Path::new("C:/Users/stele/AppData/Roaming/MetaQuotes/Terminal/D0E8209F77C8CF37AD8BF550E51FF075/MQL5/Profiles/Tester/expert_params.set")
         );
     }

     #[test]
     fn reports_dir_test() {
         let common = CommonParams::new(Path::new("C:/workdir"));
         let mut run = RunParams::new();
         run.name = "test".to_string();
         assert_eq!(reports_dir(&common , &run).as_path(),
                    PathBuf::from("C:/workdir/reports/").join("test"));

         let mut reports_path = reports_dir(&common, &run).join("USDCHF");
         reports_path.set_extension("xml");
         let reports_path = reports_path.as_os_str();

         assert_eq!(reports_path.to_string_lossy(),
                    "C:/workdir/reports/test/USDCHF.xml");
     }

     #[test]
     fn run_iter_test() {
         let mut run = RunParams::new();
         run.symbols = vec!["USDCHF", "USDJPY", "USDCAD"].iter().map(|s| s.to_string()).collect();
         let mut sym_iter = run.iter();
         assert_eq!(sym_iter.next().unwrap(), "USDCHF");
         assert_eq!(sym_iter.next().unwrap(), "USDJPY");
     }

     #[test]
     fn to_terminal_config_test() {
         let common = CommonParams::new(Path::new("C:/workdir"));
         let mut run = RunParams::new();
         run.symbols = vec!["USDCHF", "USDJPY", "USDCAD"].iter().map(|s| s.to_string()).collect();
         run.name = "test".to_string();
         let mut sym_iter = run.iter();

         assert_eq!(to_terminal_config(&common, &run , sym_iter.next().unwrap()),
"[Tester]

Expert=expert/expert.ex5
ExpertParameters=expert_params.set
Period=D1
Login=
UseLocal=1
UseRemote=1
ReplaceReport=1
ShutdownTerminal=1
Deposit=10000
Currency=USD
Leverage=100
ExecutionMode=0

Visual=0
FromDate=2017.08.01
ToDate=2019.08.20
Model=2
Optimization=1
OptimizationCriterion=6
Symbol=USDCHF
Report=C:/workdir/reports/test/USDCHF.xml"
         ); 
     }
}