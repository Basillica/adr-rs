use anyhow::Result;
use clap::Args;

use crate::utils::config::AppConfig;

#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub(crate) struct ProjectSetArgs {
    #[arg(
        long, require_equals = true, value_name = "project",
        long_help = "set project as default",
    )]
    project: String,
}

pub(crate) fn main(cfg: &mut AppConfig, args: &ProjectSetArgs) -> Result<()> {
    cfg.make_default(args.project.clone());
    Ok(())
}