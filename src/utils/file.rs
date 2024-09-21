use std::{
    fs::{create_dir_all, File, OpenOptions}, io::ErrorKind, path::{Path, PathBuf}
};
use regex::Regex;

use utils::config::AppConfig;
use std::io::Write;
use std::fs;
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag};
use crate::adr;
use crate::utils;

pub struct AdrManager<'a>{
    pub adr: &'a mut adr::adr::Adr,
    pub cfg: &'a mut AppConfig,
}

pub fn new_manager<'a>(adr: &'a mut adr::adr::Adr, cfg: &'a mut AppConfig) -> AdrManager<'a> {
    AdrManager{
        adr, cfg,
    }
}

impl<'a> AdrManager<'a> {
    pub fn init_adr(&mut self) -> Result<(), std::io::Error> {
        if self.cfg.current_project == "" {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::AddrNotAvailable,
                    "you do not have a default project selected. set a default project"
                )
            )
        }
        let init_file = format!("0001-record-architecture-decisions.md");
        if self.file_exist(Path::new(&init_file)) {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "file already exist"));
        }

        if let Some(path) = self.cfg.doc_path() {
            let adr_file_path = path.join(&init_file);
            if let Err(err) = create_dir_all(path) {
                eprintln!("Error creating data directory: {}", err);
                return Err(err);
            }

            let template = format!(
                "# {number}. {date} {title}\n\n## Status\n\nProposed\n\n## Context\n\n{desc}\n\n## Decision\n\n\n## Consequences\n\nRef: [adr-tools](https://github.com/npryce/adr-tools)",
                number=001, date={&self.adr.created_at}, title=&self.adr.title, desc=&self.adr.description
            );
        
            if let Err(err) = self.write_template_to_file(&adr_file_path, &template) {
                eprintln!("Error writing ADR template to file: {}", err);
                return Err(err);
            }
        
            println!("New ADR created: {}", adr_file_path.display());
            return Ok(());
        }

        Err(std::io::Error::new(std::io::ErrorKind::AddrNotAvailable, "file does not exist"))
    }

    pub fn create_adr(&mut self) -> Result<(), std::io::Error> {
        if let Some(adr_str) = self.cfg.next_adr_string() {
            let init_file = format!("{}-{}.md", &adr_str, utils::slugify(&self.adr.title));

            if let Some(path) = self.cfg.doc_path() {
                let adr_file_path = path.join(&init_file);
                if self.file_exist(Path::new(&init_file)) {
                    return Err(std::io::Error::new(std::io::ErrorKind::Other, "file already exist"));
                }
            
                let template = format!(
                    "# {number}. {date} {title}\n\n## Status\n\nProposed\n\n## Context\n\n{desc}\n\n## Decision\n\n\n## Consequences\n\nRef: [adr-tools](https://github.com/npryce/adr-tools)",
                    number=&adr_str, date=&self.adr.created_at, title=&self.adr.title, desc=&self.adr.description
                );
            
                if let Err(err) = self.write_template_to_file(&adr_file_path, &template) {
                    eprintln!("Error writing ADR template to file: {}", err);
                    return Err(err);
                }
            
                println!("New ADR created: {}", adr_file_path.display());
                return  Ok(())
            }
            return Err(std::io::Error::new(std::io::ErrorKind::AddrNotAvailable, "file does not exist"));
        }
        Err(std::io::Error::new(ErrorKind::NotFound, "could not create adr"))
    }
    
    fn file_exist(&self, file_path: &Path) -> bool {
        if file_path.is_file() {
            println!("The file exists!");
            return true;
        }
        false
    }
    
    pub fn write_template_to_file(&self, path: &PathBuf, template: &str) -> Result<(), std::io::Error> {
        let mut file = File::create(path)?;
        file.write_all(template.as_bytes())?;
        Ok(())
    }

    pub fn _update_template(&self, path: &PathBuf, template: &str) -> Result<(), std::io::Error> {
        // Open the file with write and truncate options
        let mut file = OpenOptions::new()
          .write(true)
          .truncate(true)
          .create(true) // Create the file if it doesn't exist
          .open(path)?;
        // Write the template content
        file.write_all(template.as_bytes())?;
      
        Ok(())
    }
    
    pub fn _read_and_parse_adr(&self, filename: &str) -> Result<String, std::io::Error> {
        let content = fs::read_to_string(filename)?;
        let parser = Parser::new(&content);
        let mut max_nesting = 0;
        let mut level = 0;
    
        let mut title = String::new();
        let in_header = false;
        let mut adr_content = String::new();
    
        for event in parser {
            match event {
                Event::Start(item) => {
                    println!(" start >>>>>>>>>>>>> here: {:?}", item);
                    level += 1;
                    max_nesting = std::cmp::max(max_nesting, level);
                }
                Event::End(item) => {
                    println!(" end >>>>>>>>>>>>> here: {:?}", item);
                    level -= 1
                }
                Event::Text(text) => {
                    Tag::Heading(HeadingLevel::H1, Some(&text), vec![""]);
                    // println!("text >>>>>>>>>>>>> code right here: {:?}", text);
                    if in_header {
                        title.push_str(&String::from_utf8_lossy(&text.as_bytes()));
                    } else {
                        adr_content.push_str(&String::from_utf8_lossy(&text.as_bytes()));
                    }
                }
                Event::Code(item) => {
                    println!(">>>>>>>>>>>>> code right here: {:?}", item)
                }
                Event::Html(item) => {
                    println!("html >>>>>>>>>>>>> {:?}", item)
                }
                Event::FootnoteReference(item) => {
                    println!("footie >>>>>>>>>>>>>>{:?}", item)
                }
                Event::SoftBreak => {
                    println!("softbreak")
                }
                Event::HardBreak=> {
                    println!("hardbreak")
                }
                Event::Rule => {
                    println!("rule")
                }Event::TaskListMarker(item) => {
                    println!("listie >>>>>>>>>>>>>>>{:?}", item)
                }
            }
        }
    
        // Check if title was found (optional, based on your requirements)
        // ...
    
        // Combine title and content for template processing
        let template = String::from(format!("**ADR Title:** {}\n\n{}", title, adr_content));
    
        // You can further process the template here...
    
        Ok(template)
    }


    fn _file_in_dir(&mut self, prefix: &str) -> Option<String> {
        if let Some(path) = self.cfg.doc_path() {
            let target_dir = Path::new(path); // Replace with your directory path
            if target_dir.is_dir() {
                for entry in fs::read_dir(target_dir).unwrap() {
                    let entry = entry.unwrap();
                    let path = entry.path();
                    if let Some(file_name) = path.file_name() {
                        let file_name_str = file_name.to_str().unwrap();
                        if file_name_str.starts_with(prefix) {
                            println!("Found file: {}", path.display());
                            return Some(file_name_str.to_string());
                        }
                    }
                }
            }
        }
        
        None
    }


    fn _all_adrs(string_list: &[String]) -> bool {
        let pattern = r"^\d+";
        for string in string_list {
            if !Regex::new(pattern).unwrap().is_match(string) {
                return false;
            }
        }
        return true;
    }

    pub fn list_adr(&mut self, path: Option<PathBuf>) -> Vec<String> {
        let target_dir = match path {
            Some(v) => v,
            None => {
                if let Some(path) = self.cfg.doc_path() {
                    path.to_path_buf()
                } else {
                    return vec![]
                }
            }
        };
        let target_dir = Path::new(&target_dir);
        let mut files: Vec<String> = vec![];
        if target_dir.is_dir() {
            for entry in fs::read_dir(target_dir).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_file() {
                  files.push(path.file_name().unwrap().to_str().unwrap().to_string());
                }
              }
        }
        files
    }
}