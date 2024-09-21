use clap::Subcommand;
use anyhow::Result;
use new::ProjectNewArgs;
use rename::ProjectRenameArgs;
use set::ProjectSetArgs;
use crate::{utils::config::AppConfig, ProjectArgs};


pub mod list;
pub mod new;
pub mod rename;
pub mod set;


#[derive(Subcommand, Debug)]
pub enum ProjectCommands {
    New(ProjectNewArgs),
    Rename(ProjectRenameArgs),
    List,
    Switch(ProjectSetArgs),
    Default,
}

pub fn project_handler(cfg: &mut AppConfig, args: &ProjectArgs) -> Result<()> {
    let cmd = &args.command.as_ref().unwrap_or(&ProjectCommands::Default);
    match cmd {
        ProjectCommands::New(args) => {
            super::project::new::main(cfg, args)?;
        }
        ProjectCommands::Rename(args) => {
            super::project::rename::main(cfg, args)?;
        },
        ProjectCommands::List => {
            super::project::list::main(cfg)?;
        },
        ProjectCommands::Switch(args) => {
            super::project::set::main(cfg, args)?;
        },
        ProjectCommands::Default => {
            println!();
        },
    }

    Ok(())
}