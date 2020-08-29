use super::*;
use std::path::PathBuf;

// terminal execution specific configuration
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CommonParams {
    pub params_file: String,
    pub wine: bool,
    pub terminal_exe: PathBuf,
    pub workdir: PathBuf,
    pub reports: PathBuf,
    pub expert: String,
    pub period: String,
    pub login: String,
    pub use_remote: bool,
    pub use_local: bool,
    pub replace_report: bool,
    pub shutdown_terminal: bool,
    pub deposit: u32,
    pub currency: String,
    pub leverage: u16,
    pub execution_mode: u8,
}

impl CommonParams {
    pub fn params_path(&self) -> PathBuf {
        let mut params_path = self.workdir.clone();
        params_path.push("MQL5/Profiles/Tester");
        params_path.push(&self.params_file);
        params_path
    }

    pub fn to_config(&self) -> String {
        format!(
            "
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
            exec_mode = self.execution_mode
        )
    }

    pub fn _new_test() -> Self {
        CommonParams {
            params_file: "expert_params.set".to_string(),
            wine: false,
            terminal_exe: PathBuf::from(r"C:\terminal64.exe"),
            workdir: PathBuf::from(r"C:\workdir"),
            reports: PathBuf::from("reports"),
            expert: r"expert\expert.ex5".to_string(),
            period: "D1".to_string(),
            login: "1234".to_string(),
            use_remote: true,
            use_local: true,
            replace_report: true,
            shutdown_terminal: true,
            deposit: 10000,
            currency: "USD".to_string(),
            leverage: 100,
            execution_mode: 0,
        }
    }
}
