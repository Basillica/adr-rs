use anyhow::Result;
use clap::Args;

use crate::utils::config::{AppConfig, Project};

#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub(crate) struct ProjectNewArgs {
    #[arg(
        long, require_equals = true, value_name = "path",
        long_help = "the path to where the new project will be created",
        help = "the path to where the new project will be created",
        short = 'P',
    )]
    path: String,
    #[arg(
        long, require_equals = true, value_name = "name",
        long_help = "the name of the new project",
        help = "the name of the new project",
        short = 'N',
    )]
    name: String,
}

pub(crate) fn main(cfg: &mut AppConfig, args: &ProjectNewArgs) -> Result<()> {
    let mut prj = Project::default();
    prj.path = args.path.clone().into();
    cfg.new_project(args.name.clone(), prj);
    Ok(())
}