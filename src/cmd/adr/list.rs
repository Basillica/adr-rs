use anyhow::Result;
use utils::{file::{new_manager, AdrManager}, config::AppConfig};
use std::path::PathBuf;
use clap::Args;
use adr::{adr::Adr, adr::Builder};


use crate::utils;
use crate::adr;

#[derive(Debug, Args)]
#[command(version, about, long_about = None, short_flag = 'l')]
pub(crate) struct ListArgs {
    /// Directory to initialize
    #[arg(short = 'd', help = "The directory where the adr files are located")]
    adr_dir: Option<PathBuf>
}


pub(crate) fn main(args: &ListArgs, cfg: &mut AppConfig) -> Result<()> {
    // if args.adr_dir.is_some() {
    //     cfg.update_path(args.adr_dir.clone().unwrap());
    // }
    let mut adr = Adr::new();
    let mut manager: AdrManager<'_> = new_manager(&mut adr, cfg);
    let files = manager.list_adr(args.adr_dir.clone());
    println!("{:#?}", files);
    Ok(())
}