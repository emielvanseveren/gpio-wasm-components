use anyhow::Result;
use clap::Parser;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[structopt(name = "run", trailing_var_arg = true)]
pub struct VersionCommand;

impl VersionCommand {
    pub fn execute(self) -> Result<()> {
        println!("{}", VERSION);
        Ok(())
    }
}
