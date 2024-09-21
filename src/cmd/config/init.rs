use anyhow::Result;
use clap::Args;

#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub(crate) struct InitConfigArgs {
    #[arg(default_value = "docs/adr")]
    data_dir: String,
}

pub(crate) fn main(args: &InitConfigArgs) -> Result<()> {
    println!("right here my lovelies {}", args.data_dir);
    Ok(())
}