use clap::Parser;
use rcli::{Opts, SubCommand};
use anyhow::Result;
fn main() -> Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(csv_opts) => {
            rcli::process_csv(&csv_opts.input, csv_opts.output, csv_opts.format)?;
        }
    }
    Ok(())
}
