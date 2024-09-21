use anyhow::Result;
use clap::Args;

#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub(crate) struct StartConfigArgs {
    #[arg(default_value = "docs/adr")]
    data_dir: String,
}

pub(crate) fn main(args: &StartConfigArgs) -> Result<()> {
    println!("{}", args.data_dir);
    Ok(())
}