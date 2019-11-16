#![allow(dead_code)]
#![allow(unused_variables)]

use super::params::*;
use super::xml_reader;
use super::xml_reader::*;

use std::fs::{self, File};
use std::future::Future;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;

use anyhow::{Context, Result};

// use heim_common::prelude::futures::stream::*;
// use futures::prelude::*;
use std::process::{Child, Command, ExitStatus};
// use heim::process::Process;

#[derive(Debug)]
pub struct BacktestRunner {
    common: CommonParams,
    run: RunParams,
    // symbol_iter : Iterator<&String>,
}

impl BacktestRunner {
    pub fn new(run: RunParams, common: CommonParams) -> BacktestRunner {
        BacktestRunner {
            common: common,
            run: run,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.run.iter()
    }

    fn write_indi_params(&self) -> Result<()> {
        // TODO mkdir
        let mut file = File::create(self.common.params_path())?;
        file.write_all(self.run.to_params_config().unwrap().as_bytes())?;
        Ok(())
    }

    fn write_terminal_config(&self, symbol: &String) -> Result<()> {
        // TODO mkdir
        let mut file = File::create(self.common.workdir.join("terminal.ini").as_path())?;
        file.write_all(to_terminal_config(&self.common, &self.run, symbol)?.as_bytes())?;
        Ok(())
    }

    fn run_terminal(&self) -> Result<ExitStatus> {
        let mut cmd = Command::new(
            self.common
                .terminal_exe
                .as_os_str()
                .to_str()
                .context("conversion error for terminal.exe path")?,
        );
        cmd.arg(format!(
            "/config:{}",
            self.common
                .workdir
                .join("terminal.ini")
                .as_os_str()
                .to_str()
                .context("conversion error in terminal.ini path")?
        ));
        debug!("running terminal: {:?}", cmd);

        let output = cmd.output().context("Terminal Command execution failed")?;
        debug!(
            "Terminal out: {}{}",
            String::from_utf8_lossy(&output.stdout).trim(),
            String::from_utf8_lossy(&output.stdout).trim()
        );
        Ok(output.status)
    }

    pub fn run_backtest(&self) -> Result<bool> {
        self.write_indi_params()?;
        fs::create_dir_all(get_reports_dir(&self.common, &self.run)?)?;
        for symbol in self.run.symbols.iter() {
            self.write_terminal_config(&symbol)?;
            self.run_terminal()?;
            let results = self.collect_report(&symbol)?;
            self.delete_report(&symbol)?;
        }
        // self.push_results_to_database().context(DbError)?;
        fs::remove_dir(get_reports_dir(&self.common, &self.run)?)?;
        Ok(true)
    }

    fn delete_report(&self, symbol: &String) -> Result<()> {
        let report = get_reports_path(&self.common, &self.run, symbol)?;
        debug!("deleting report {}", report.to_string_lossy());
        fs::remove_file(report)?;
        Ok(())
    }

    fn collect_report(&self, symbol: &String) -> Result<Vec<ResultRow>> {
        let results = read_results_xml(get_reports_path(&self.common, &self.run, symbol)?)?;
        trace!("{:?}", results);
        Ok(results)
    }

    /* fn push_results_to_database(&self) -> Result<(), Error> {
     *     unimplemented!();
     * } */
}

/* async fn find_process(name : String) -> heim::process::ProcessResult<Process>
 * {
 *     // let mut processes : heim_common::prelude::futures::stream::Stream<Item = Process> = heim::process::processes();
 *     let mut processes = heim::process::processes();
 *     while let Some(process) = processes.poll_next().await {
 *         let process = process?;
 *         if process.name().await.unwrap_or_else(|_| "".to_string()) == name {
 *             println!("found {} with pid: {}", name, process.pid());
 *             return Ok(process);
 *         }
 *
 *     }
 *     Err(heim::process::ProcessError::NoSuchProcess(0))
 * } */

// pub async fn run() -> heim::process::ProcessResult<()> {
/* if let Ok(terminal_process) = find_process("terminal64.exe".to_string()).await {
 *     println!("closing terminal with pid {}", terminal_process.pid());
 *     terminal_process.terminate().await?
 * } */

// Ok(())
// }

#[cfg(test)]
mod test {
    use super::*;

    /*     #[tokio::test]
     *     async fn test_find_process() {
     *         let mut s = Command::new("/usr/bin/sleep")
     *             .arg("10")
     *             .spawn().unwrap();
     *         let sl = find_process("sleep".to_string()).await;
     *         assert!(sl.is_ok());
     *         // assert!(sl.unwrap().terminate().await.is_ok());
     *
     *         s.wait().unwrap();
     *     } */
}
