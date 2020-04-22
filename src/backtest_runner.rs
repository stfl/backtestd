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
use chrono::offset::LocalResult;
use chrono::prelude::*;

// use heim_common::prelude::futures::stream::*;
// use futures::prelude::*;
use std::process::{Child, Command, ExitStatus, Stdio};
// use heim::process::Process;
use std::{thread, time};

#[derive(Debug)]
pub struct BacktestRunner {
    common: CommonParams,
    run: RunParams,
    // symbol_iter : Iterator<&String>,
}

impl BacktestRunner {
    pub fn new(run: RunParams, common: &CommonParams) -> BacktestRunner {
        BacktestRunner {
            common: common.clone(),
            run: run,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.run.iter()
    }

    fn write_indi_params(&self) -> Result<()> {
        debug!("writing {:?}", self.common.params_path());
        let mut file = File::create(self.common.params_path())?;
        file.write_all(self.run.to_params_config()?.as_bytes())?;
        Ok(())
    }

    fn write_terminal_config(&self) -> Result<()> {
        debug!("writing {:?}", self.common.params_path());
        let mut file = File::create(self.common.workdir.join("terminal.ini").as_path())?;
        file.write_all(to_terminal_config(&self.common, &self.run)?.as_bytes())?;
        Ok(())
    }

    fn run_terminal(&self) -> Result<ExitStatus> {
        let mut cmd: Command;
        if self.common.wine {
            cmd = Command::new("wine");
            cmd.arg(
                self.common
                    .terminal_exe
                    .as_os_str()
                    .to_str()
                    .context("conversion error for terminal.exe path")?,
            );
        } else {
            cmd = Command::new(
                self.common
                    .terminal_exe
                    .as_os_str()
                    .to_str()
                    .context("conversion error for terminal.exe path")?,
            );
        }
        cmd.arg(format!("/config:{}", "terminal.ini"))
           .current_dir(&self.common.workdir);
        debug!("running terminal: {:?}", cmd);

        let mut child = cmd.spawn().context("Command spawning failed")?;
        let ret = child.wait().context("Waiting for Command failed");
        if self.common.wine {
            // sleep a little for wine to properly terminate
            thread::sleep(time::Duration::from_millis(5000));
        }
        ret
    }

    pub fn run_backtest(&self, keep_reports: bool) -> Result<Vec<BacktestResult>> {
        self.write_indi_params()?;
        fs::create_dir_all(get_reports_dir(&self.common, &self.run)?)?;
        self.write_terminal_config()?;
        let _ = self.delete_terminal_log();
        self.run_terminal()?;
        let _ = self.save_terminal_log();
        let results = self.collect_report()?;

        if !keep_reports {
            // self.delete_report()?;
            // self.push_results_to_database().context(DbError)?;
            // TODO cannot delete if not empty
            // fs::remove_dir(get_reports_dir(&self.common, &self.run)?)?;
        }
        Ok(results)
    }

    fn delete_terminal_log(&self) -> Result<()> {
        let log_path = self.get_original_log_path();
        if let Err(e) = fs::remove_file(&log_path) {
            warn!("removing terminal log failed {:?} {}", log_path, e);
        }
        Ok(())
    }

    fn save_terminal_log(&self) -> Result<PathBuf> {
        let run_log = get_reports_path(&self.common, &self.run)?.with_extension("log");
        if fs::rename(self.get_original_log_path(), &run_log).is_ok() {
            match fs::read(&run_log) {
                Ok(s) => debug!("Tester output:\n{}", String::from_utf8_lossy(&s)),
                Err(e) => error!("reading {:?} failed: {:?}", &run_log, e),
            };
        } else {
            error!("copying log failed");
            return Err(anyhow!("copying log failed"));
        }
        Ok(run_log)
    }

    fn get_original_log_path(&self) -> PathBuf {
        self.common
            .workdir
            .join("Tester/logs")
            .join(Local::today().format("%Y%m%d").to_string())
            .with_extension("log")
    }

    fn delete_report(&self) -> Result<()> {
        let report = get_reports_path(&self.common, &self.run)?;
        debug!("deleting report {}", report.to_string_lossy());
        fs::remove_file(report)?;
        Ok(())
    }

    fn collect_report(&self) -> Result<Vec<BacktestResult>> {
        let results = read_results_xml(
            &self.run.indi_set,
            get_reports_path(&self.common, &self.run)?,
        )?;
        // TODO trace! does not work anymore when includeing actix-web
        // trace!("{:?}", results);
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
