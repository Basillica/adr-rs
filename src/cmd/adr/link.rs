use anyhow::Result;
use clap::Args;

#[derive(Debug, Args)]
#[command(version, about, long_about = None, short_flag = 'k')]
pub(crate) struct LinkArgs {
    /// Directory to initialize
    #[arg(default_value = "doc/adr")]
    source: String,
}

pub(crate) fn main(args: &LinkArgs) -> Result<()> {
    println!("{}", args.source);
    Ok(())
}