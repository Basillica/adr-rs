use anyhow::Result;
use utils::{file::{new_manager, AdrManager}, config::AppConfig};
use clap::Args;
use adr::{adr::Adr, adr::Builder};


use crate::utils;
use crate::adr;

#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub(crate) struct ListArgs {
    /// Directory to initialize
    #[arg(default_value = "doc/adr", short = 'd', help = "The directory where the adr files are located")]
    adr_dir: String,
    #[arg(short = 'n', help = "The number of the adr file")]
    number: Option<i64>,
    #[arg(short = 'f', help = "The name of the adr file")]
    name: Option<String>,

}

pub(crate) fn main(args: &ListArgs, cfg: &mut AppConfig) -> Result<()> {
    if args.number.is_some() {
        let num = utils::number_to_string(args.number.unwrap());
        println!("{:?}", args.number);
    }
    if args.name.is_some() {
        println!("{:?}", args.name);
    }
    
    let mut adr = Adr::new();
    let mut manager: AdrManager<'_> = new_manager(&mut adr, cfg, None);
    println!("{}", args.adr_dir);
    Ok(())
}