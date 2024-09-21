use anyhow::Result;
use clap::Subcommand;

use link::LinkArgs;
use list::ListArgs;
use new::NewArgs;
use unlink::UnlinkArgs;
use update::UpdateArgs;

use crate::{utils::config::AppConfig, AdrArgs};


pub mod init;
pub mod link;
pub mod list;
pub mod new;
pub mod unlink;
pub mod update;



#[derive(Subcommand, Debug)]
pub enum AdrCommands {
    Init,
    List(ListArgs),
    New(NewArgs),
    Unlink(UnlinkArgs),
    Link(LinkArgs),
    Update(UpdateArgs),
    Default,
}


pub fn adr_handler(cfg: &mut AppConfig, args: &AdrArgs) -> Result<()> {
    let cmd = &args.command.as_ref().unwrap_or(&AdrCommands::Default);
    match cmd {
        AdrCommands::Init => {
            super::adr::init::main(cfg)?;
        }
        AdrCommands::New(args) => {
            super::adr::new::main(args, cfg)?;
        },
        AdrCommands::Link(args) => {
            super::adr::link::main(args)?;
        },
        AdrCommands::Unlink(args) => {
            super::adr::unlink::main(args)?;
        },
        AdrCommands::List(args) => {
            super::adr::list::main(args, cfg)?;
        },
        AdrCommands::Update(args) => {
            super::adr::update::main(args, cfg)?;
        },
        AdrCommands::Default => {
            println!();
        },
    }

    Ok(())
}