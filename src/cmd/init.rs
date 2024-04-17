use anyhow::Result;
use clap::Args;
use std::path::PathBuf;
use adr::{adr::Adr, adr::Builder};
use utils::{config::AppConfig, file::{new_manager, AdrManager}};

use crate::adr;
use crate::utils;


#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub(crate) struct InitArgs {
    #[arg(default_value = "docs/adr")]
    data_dir: PathBuf
}

pub(crate) fn main(args: &InitArgs, cfg: &mut AppConfig) -> Result<()> {
    let mut adr = Adr::new();
    adr
        .set_content("The content of the ADR")
        .set_description("A short description")
        .set_title("Record architecture decisions");

    let mut manager: AdrManager<'_> = new_manager(&mut adr, cfg, Some(args.data_dir.clone()));
    match manager.init_adr() {
        Ok(_) => {
            cfg.increment_count();
        },
        Err(e) => println!("{:?}", e),
    };
    Ok(())
}

// fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
//     for line in content.lines() {
//         if line.contains(pattern) {
//             writeln!(writer, "{}", line);
//         }
//     }
// }

// #[test]
// fn find_a_match() {
//     let mut result = Vec::new();
//     find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
//     assert_eq!(result, b"lorem ipsum\n");
// }
