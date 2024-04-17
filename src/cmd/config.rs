use anyhow::Result;
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub(crate) struct ConfigArgs {
    /// Directory to initialize
    #[arg(default_value = "doc/adr")]
    source: String,
    #[command(subcommand)]
    commands: Commands,
}


#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub(crate) struct InitConfigArgs {
    #[arg(default_value = "docs/adr")]
    data_dir: String,
}

#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub(crate) struct StartConfigArgs {
    #[arg(default_value = "docs/adr")]
    data_dir: String,
}


#[derive(Subcommand, Debug)]
enum Commands {
    Init(InitConfigArgs),
    Start(StartConfigArgs),
}

pub(crate) fn main(args: &ConfigArgs) -> Result<()> {
    println!("{}", args.source);
    Ok(())
}