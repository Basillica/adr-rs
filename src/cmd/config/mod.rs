use anyhow::Result;
use clap::Subcommand;
use init::InitConfigArgs;
use start::StartConfigArgs;

use crate::{utils::config::AppConfig, ConfigArgs};


pub mod init;
pub mod start;


#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    Init(InitConfigArgs),
    Start(StartConfigArgs),
    Default,
}

pub fn config_handler(cfg: &mut AppConfig, args: &ConfigArgs) -> Result<()> {
    let cmd = &args.command.as_ref().unwrap_or(&ConfigCommands::Default);
    match cmd {
        ConfigCommands::Init(args) => {
            super::config::init::main(args)?;
        }
        ConfigCommands::Start(args) => {
            super::config::start::main(args)?;
        },
        ConfigCommands::Default => {
            println!();
        },
    }

    Ok(())
}