use anyhow::Result;
use clap::Parser;
use xtask_common::{clap, Cli, Empty};

fn main() -> Result<()> {
    let cli: Cli<Empty> = Cli::parse();

    cli.command.execute("verdi")?;

    Ok(())
}
