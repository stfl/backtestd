use super::params::*;
use crate::results::xml_reader::*;
use crate::results::ResultRow;

use anyhow::{Context, Result};
use chrono::prelude::*;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use std::process::{Command, ExitStatus};
use std::{thread, time};

#[derive(Debug)]
pub struct BacktestRunner {
    common: CommonParams,
    run: RunParams,
}

impl BacktestRunner {
    pub fn new(run: RunParams, common: &CommonParams) -> BacktestRunner {
        BacktestRunner {
            common: common.clone(),
            run,
        }
    }

    fn write_indi_params(&self) -> Result<()> {
        debug!("writing {:?}", self.common.params_path());
        let mut file = File::create(self.common.params_path())?;
        file.write_all(self.run.to_param_string().as_bytes())?;
        Ok(())
    }

    fn write_terminal_config(&self) -> Result<()> {
        debug!("writing {:?}", self.common.params_path());
        let mut file = File::create(self.common.workdir.join("terminal.ini").as_path())?;
        file.write_all(to_terminal_config(&self.common, &self.run)?.as_bytes())?;
        Ok(())
    }

    pub fn run(&self) -> Result<ExitStatus> {
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

    pub fn prepare_files(&self) -> Result<()> {
        self.write_indi_params()?;
        fs::create_dir_all(get_reports_dir(&self.common)?)?;
        self.write_terminal_config()?;
        let _ = self.delete_terminal_log();
        Ok(())
    }

    fn delete_terminal_log(&self) -> Result<()> {
        let log_path = self.get_original_log_path();
        if let Err(e) = fs::remove_file(&log_path) {
            warn!("removing terminal log failed {:?} {}", log_path, e);
        }
        Ok(())
    }

    fn save_terminal_log(&self) -> Result<PathBuf> {
        let run_log = get_reports_full_path(&self.common, &self.run)?.with_extension("log");
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

    fn delete_xml_report(&self) -> Result<()> {
        let report = get_reports_full_path(&self.common, &self.run)?;
        debug!("deleting report {}", report.to_string_lossy());
        fs::remove_file(report)?;
        Ok(())
    }

    pub fn _read_results(&self) -> Result<Vec<ResultRow>> {
        let _ = self.save_terminal_log();
        let results = _read_results_xml(get_reports_full_path(&self.common, &self.run)?)?;
        // TODO trace! does not work anymore when includeing actix-web
        // trace!("{:?}", results);
        Ok(results)
    }

    pub fn convert_results_to_csv(&self) -> Result<i32> {
        let _ = self.save_terminal_log();
        let reports_path = get_reports_full_path(&self.common, &self.run)?;
        let ret = read_results_xml_to_csv(&reports_path, &reports_path.with_extension("csv"));
        ret
    }

    pub fn cleanup(&self) -> Result<()> {
        self.delete_xml_report()?;
        self.delete_terminal_log()
    }

    pub fn _remove_sqlite_file(&self) -> std::result::Result<(), std::io::Error> {
        std::fs::remove_file(
            self.common
                .workdir
                .join(Path::new("MQL5/Files"))
                .join(&self.run.name)
                .with_extension("sqlite"),
        )
    }
}

pub fn execute_run_queue(config: &CommonParams, runs: &Vec<RunParams>) -> Result<()> {
    for r in runs {
        debug!(
            "Run: {:?}\nInputs: {}",
            r,
            r.indi_set.count_inputs_crossed()
        );
        let runner = BacktestRunner::new(r.clone(), &config);
        // if let Err(err) = runner.remove_sqlite_db() { // TODO this should be done from within the Expert
        //     warn!("delete sqlite failed {:?}", err);
        // };
        runner.prepare_files().context("prepare failed")?;
        runner.run().context("run failed")?;
        runner
            .convert_results_to_csv()
            .context("convert to csv failed")?;
        runner.cleanup().context("cleanup failed")?;
    }
    Ok(())
}

pub fn get_csv_filenames_from_queue(
    config: &CommonParams,
    runs: &Vec<RunParams>,
) -> Vec<PathBuf> {
    runs.iter()
        .map(|r| {
            config
                .reports
                .join(r.get_reports_filename())
                .with_extension("csv")
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_csv_filenames_from_queue() {
        let common = CommonParams::_new_test();
        assert_eq!(
            get_csv_filenames_from_queue(&common, &Vec::new()),
            Vec::<PathBuf>::new()
        );

        let runs = vec![RunParams::_new_test(1)];

        assert_eq!(
            get_csv_filenames_from_queue(&common, &runs),
            [PathBuf::from(r"reports/test_USDCHF.csv")]
        );

        let mut runs = vec![RunParams::_new_test(1), RunParams::_new_test(1)];
        runs[1].symbols = vec!["NZDAUD".into()];

        assert_eq!(
            get_csv_filenames_from_queue(&common, &runs),
            [
                PathBuf::from(r"reports/test_USDCHF.csv"),
                PathBuf::from(r"reports/test_NZDAUD.csv")
            ]
        );
        // FIXME this is not working.. never fails!!!
    }
}
