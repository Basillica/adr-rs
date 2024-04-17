use anyhow::Result;
use clap::Args;

#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub(crate) struct ExportArgs {
    /// Directory to initialize
    #[arg(default_value = "doc/adr")]
    source: String,
}

pub(crate) fn main(args: &ExportArgs) -> Result<()> {
    println!("{}", args.source);
    Ok(())
}