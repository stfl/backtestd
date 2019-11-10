#![allow(dead_code)]
#![allow(unused_variables)]

use super::params::*;
use super::xml_reader;
use super::xml_reader::*;

use std::fs::File;
use std::future::Future;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;

use snafu::{Backtrace, ResultExt, Snafu};

// use heim_common::prelude::futures::stream::*;
// use futures::prelude::*;
use std::process::{Child, Command, ExitStatus};
// use heim::process::Process;

/* impl From<io::Error> for Error::IoError {
 *     fn from(err: io::Error) -> Self {
 *         Error::IoError::new(err.description())
 *     }
 * } */

/* impl From<xml_reader::Error> for XmlReaderError {
 *     fn from(err: xml_reader::Error) -> Self {
 *         XmlReaderError::new(err.description())
 *     }
 * } */

#[derive(Debug, Snafu)]
pub enum Error {
    // #[snafu(display("{}", description))]
    IoError { source: io::Error },

    // #[snafu(display(""))]
    XmlReaderError { source: xml_reader::Error },
}

type Result<T, E = Error> = std::result::Result<T, E>;

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
        let mut file = File::create(self.common.params_path()).context(IoError)?;
        file.write_all(self.run.to_params_config().as_bytes())
            .context(IoError {})?;
        Ok(())
    }

    fn write_terminal_config(&self, symbol: &String) -> Result<(), io::Error> {
        // TODO mkdir
        let mut file = File::create(self.common.workdir.join("terminal.ini").as_path())?;
        file.write_all(to_terminal_config(&self.common, &self.run, symbol).as_bytes())?;
        Ok(())
    }

    /* fn run_terminal(&mut self) -> Result<(), io::Error> {
     *     let mut terminal_cmd = Command::new(self.common.terminal_exe.to_str()
     *                                         .expect(format!("terminal_exe {:?} cannot convert to str", self.common.terminal_exe).as_str()));
     *     self.process = Some(terminal_cmd.spawn()?);
     *     Ok(())
     * } */

    fn run_terminal(&self) -> Result<ExitStatus, io::Error> {
        Command::new(
            self.common.terminal_exe.to_str().expect(
                format!(
                    "terminal_exe {:?} cannot convert to str",
                    self.common.terminal_exe
                )
                .as_str(),
            ),
        )
        .status()
    }

    pub async fn run_backtest(&self) -> Result<bool> {
        for symbol in self.run.symbols.iter() {
            self.write_terminal_config(&symbol).context(IoError)?;
            self.run_terminal().context(IoError)?;
            let results = self.collect_results(&symbol).context(XmlReaderError)?;
            self.delete_results(&symbol).context(IoError)?;
        }
        // self.push_results_to_database().context(DbError)?;
        Ok(true)
    }

    fn delete_results(&self, symbol: &String) -> Result<(), io::Error> {
        let report = get_reports_path(&self.common, &self.run, symbol);
        println!("deleting report {}", report.to_string_lossy());
        Ok(())
    }

    fn collect_results(&self, symbol: &String) -> Result<Vec<ResultRow>, xml_reader::Error> {
        Ok(read_results_xml(get_reports_path(
            &self.common,
            &self.run,
            symbol,
        ))?)
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
