use anyhow::Result;
use clap::Args;
use adr::{adr::Adr, adr::Builder};
use utils::{file::{new_manager, AdrManager}, config::AppConfig};


use crate::utils;
use crate::adr;


#[derive(Debug, Args)]
#[command(version, about, long_about = None, short_flag = 'n')]
pub(crate) struct NewArgs {
    /// Name of adr to initialize
    #[arg(short = 'a', help = "The name of the adr file")]
    adr_name: String,
    /// The title of the new adr
    #[arg(short = 't', help = "The title of the adr file")]
    title: String,
    /// The description of the ADR
    #[arg(default_value = "A short description", short = 'd', help = "The description of the adr file")]
    description: String,
    /// The content of the ADR
    #[arg(default_value = "The content of the ADR", short = 'c', help = "The content of the adr file")]
    content: String,
}

pub(crate) fn main(args: &NewArgs, cfg: &mut AppConfig) -> Result<()> {
    let mut adr = Adr::new();
    adr
        .set_content(&args.content)
        .set_description(&args.description)
        .set_title(&args.title);
    let mut manager: AdrManager<'_> = new_manager(&mut adr, cfg);
    match manager.create_adr() {
        Ok(_) => cfg.increment_count(),
        Err(e) => println!("{:?}", e),
    };
    Ok(())
}