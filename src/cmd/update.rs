use anyhow::Result;
use clap::Args;
use utils::config::AppConfig;
use std::path::PathBuf;
use std::fs::{self};


use crate::utils;

#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub(crate) struct UpdateArgs {
    /// The name of the ADR file to be updated
    file_name: Option<String>,
    /// The number-prefix of the filename of the ADR file to be updated
    file_number: Option<i32>,
    /// The new title of the adr
    title: Option<String>,
    /// The new description
    description: Option<String>,
    /// The new content
    content: Option<String>,
}

impl UpdateArgs {
    fn has_valid_identifier(&self) -> bool {
        self.file_name.is_some() || self.file_number.is_some()
    }

    pub fn perform_update(&self) {
        if let Some(name) = &self.file_name {
          // Perform operation using file_name
          println!("Update by filename: {}", name);
        } else if let Some(number) = self.file_number {
          // Perform operation using file_number
          println!("Update by file_number: {}", number);
        } else {
          println!("Update operation requires either file_name or file_number");
        }
    }

    pub fn _get_file_name(&self) {

    }

    fn _get_full_filename(file_id: &str, base_path: &PathBuf) -> Result<String, std::io::Error> {
        // Construct the full filename with leading zeros and extension
        let full_filename = format!("{:04}-some-file.md", file_id);
        // Combine base path and filename
        let path = base_path.join(&full_filename);
        // Check if the file exists
        if fs::metadata(&path).is_ok() {
          Ok(full_filename)
        } else {
          Ok(String::from("")) // Return empty string for non-existent file
        }
    }

    fn _find_file_by_prefix(_file_id: &str, base_path: &PathBuf) -> Option<String> {
        // Iterate through entries in the base path
        let paths = fs::read_dir(base_path).unwrap();
        for path in paths {
            println!("Name: {}", path.unwrap().path().display())
        }
        // No file found with the prefix
        None
    }

    
}

pub(crate) fn main(args: &UpdateArgs, _cfg: &mut AppConfig) -> Result<()> {
    println!("{:?}", args.file_name);
    if args.has_valid_identifier() {
        args.perform_update()
    }
    Ok(())
}
