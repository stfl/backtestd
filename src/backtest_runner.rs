#[allow(dead_code)]

use super::params::*;

use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug)]
pub struct BacktestRunner {
    common : CommonParams,
    run : RunParams,
    // symbol_iter : Iterator<&String>,
}

impl BacktestRunner {
    pub fn new(run : RunParams, workdir : &Path) -> BacktestRunner {
        BacktestRunner{
            common : CommonParams::new(workdir),
            run : run
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.run.iter()
    }

    fn write_indi_params(&self) -> Result<(), io::Error> {
        // TODO mkdir
        let mut file = File::create(self.common.params_path())?;
        file.write_all(self.run.to_params_config().as_bytes())?;
        Ok(())
    }

    pub fn write_terminal_config(&self, symbol : &String) -> Result<(), io::Error> {
        // TODO mkdir
        let mut file = File::create(self.common.workdir.join("terminal.ini").as_path())?;
        file.write_all(to_terminal_config(&self.common, &self.run, symbol).as_bytes());
        Ok(())
    }

    fn start_terminal(&self) -> Result<(), io::Error> {
        unimplemented!();
    }

    fn collect_results(&self) -> Result<(), io::Error> {
        unimplemented!();
    }

    fn push_results_to_database(&self) -> Result<(), io::Error> {
        unimplemented!();
    }

}
