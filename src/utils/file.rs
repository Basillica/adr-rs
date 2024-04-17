use std::{
    fs::{create_dir_all, File, OpenOptions},
    path::{Path, PathBuf},
};

use utils::config::AppConfig;
use std::io::Write;
use std::fs;
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag};
use crate::adr::{self};
use crate::utils;

pub struct AdrManager<'a>{
    pub adr: &'a mut adr::adr::Adr,
    pub cfg: &'a mut AppConfig,
    pub path_buf: Option<PathBuf>,
}

pub fn new_manager<'a>(adr: &'a mut adr::adr::Adr, cfg: &'a mut AppConfig, path_buf: Option<PathBuf>) -> AdrManager<'a> {
    AdrManager{
        adr, cfg, path_buf,
    }
}

impl<'a> AdrManager<'a> {
    pub fn init_adr(&mut self) -> Result<(), std::io::Error> {
        let init_file = format!("0001-record-architecture-decisions.md");
        let adr_file_path = &self.cfg.doc_path().join(&init_file);
        if self.file_exist(Path::new(&init_file)) {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "file already exist"));
        }
        if let Err(err) = create_dir_all(&self.cfg.doc_path()) {
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
        Ok(())
    }

    pub fn create_adr(&mut self) -> Result<(), std::io::Error> {
        let init_file = format!("{}-{}.md", &self.cfg.next_adr_string(), utils::slugify(&self.adr.title));
        let adr_file_path = &self.cfg.doc_path().join(&init_file);
        if self.file_exist(Path::new(&init_file)) {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "file already exist"));
        }
    
        let template = format!(
            "# {number}. {date} {title}\n\n## Status\n\nProposed\n\n## Context\n\n{desc}\n\n## Decision\n\n\n## Consequences\n\nRef: [adr-tools](https://github.com/npryce/adr-tools)",
            number=&self.cfg.next_adr_string(), date=&self.adr.created_at, title=&self.adr.title, desc=&self.adr.description
        );
    
        if let Err(err) = self.write_template_to_file(&adr_file_path, &template) {
            eprintln!("Error writing ADR template to file: {}", err);
            return Err(err);
        }
    
        println!("New ADR created: {}", adr_file_path.display());
        Ok(())
    }
    
    fn file_exist(&self, file_path: &Path) -> bool {
        if file_path.is_file() {
            println!("The file exists!");
            return true;
        }
        println!("The path exists, but it's not a file.");
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
}