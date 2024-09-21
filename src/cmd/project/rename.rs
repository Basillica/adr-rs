use anyhow::Result;
use clap::Args;

use crate::utils::config::AppConfig;

#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub(crate) struct ProjectRenameArgs {
    #[arg(
        long, require_equals = true, value_name = "path",
        long_help = "the path to where the new project will be created",
    )]
    project: String,
    #[arg(
        long, require_equals = true, value_name = "name",
        long_help = "the name of the new project",
    )]
    name: String,
}

pub(crate) fn main(cfg: &mut AppConfig, args: &ProjectRenameArgs) -> Result<()> {
    cfg.rename_project(args.project.clone(), args.name.clone());
    Ok(())
}