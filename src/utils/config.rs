use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::collections::HashMap;
use crate::utils;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AppConfig {
    app_name: String,
    pub current_project: String,
    projects: HashMap<String, Project>
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Project {
    pub current_adr_number: i16,
    pub path: PathBuf,
    created_at: String,
    pub updated_at: String,
}

impl Default for Project {
    fn default() -> Self {
        Project{
            current_adr_number: 0,
            path: "".into(),
            created_at: Utc::now().to_rfc2822(),
            updated_at: Utc::now().to_rfc2822(),
        }
    }
}


impl ::std::default::Default for AppConfig {
    fn default() -> Self {
        let app_name: &'static str = env!("CARGO_PKG_NAME");
        Self {
            app_name: app_name.to_string(),
            current_project: String::new(),
            projects: HashMap::new(),
        }
    }
}

impl AppConfig{
    pub fn _update(&mut self, cfg: &AppConfig) {
        match confy::store(&self.app_name, None, cfg){
            Ok(_) => println!("app config successfully updated 000"),
            Err(e) => println!("there was an error updating app config. error: {:?}", e),
        };
    }

    pub fn _update_path(&mut self, path: String) {
        let config = AppConfig{
            app_name: self.app_name.clone(),
            current_project: self.current_project.clone(),
            projects: self.projects.clone(),
        };
        match confy::store(&self.app_name, None, config){
            Ok(_) => println!("app config successfully updated 111"),
            Err(e) => println!("there was an error updating app config. error: {:?}", e),
        };
    }

    pub fn doc_path(&mut self) -> Option<&PathBuf> {
        if let Some(prj)  = self.projects.get(&self.current_project) {
            return Some(&prj.path)
        };
        None
    }

    pub fn next_adr_string(&mut self) -> Option<String> {
        if let Some(project) = self.projects.get_mut(&self.current_project) {
            return Some(utils::number_to_string(project.current_adr_number+1));
        } else {
            println!("Project {} not found.", self.current_project);
            return None;
        };
    }

    pub fn _current_adr_number(&mut self) -> Option<i16> {
        if let Some(project) = self.projects.get_mut(&self.current_project) {
            return Some(project.current_adr_number);
        } else {
            println!("Project {} not found.", self.current_project);
            return None;
        };
    }

    pub fn increment_count(&mut self) {
        if let Some(project) = self.projects.get_mut(&self.current_project) {
            project.current_adr_number+=1;
            match confy::store(&self.app_name, None, &self){
                Ok(_) => println!("app config successfully updated"),
                Err(e) => println!("there was an error incrementing app adr count. error: {:?}", e),
            };
        } else {
            println!("Project {} not found.", self.current_project);
        };        
    }

    pub fn list_projects(&mut self) {
        if self.projects.len() == 0 {
            println!("you have not creatd any projects yet");
            return;
        }
        for (key, project) in self.projects.iter() {
            println!("project name: {}: adrs: {}", key, project.current_adr_number); 
        }
    }

    pub fn new_project(&mut self, key: String, prj: Project) {
        if self.projects.get(&key).is_some() {
            return println!("there is already a project with this name created")
        } else {
            self.projects.insert(key, prj);
            match confy::store(&self.app_name, None, &self){
                Ok(_) => println!("app config successfully updated"),
                Err(e) => println!("there was an error saving new project error: {:?}", e),
            };
        }
    }

    pub fn rename_project(&mut self, key: String, new_name: String) {
        if let Some(prj) = self.projects.get(&key) {
            self.projects.insert(new_name, prj.to_owned());
        }
        self.projects.remove(&key);
        match confy::store(&self.app_name, None, &self){
            Ok(_) => println!("app config successfully updated"),
            Err(e) => println!("there was an error updating project name: {:?}", e),
        };
    }

    pub fn make_default(&mut self, key: String) {
        if self.projects.get(&key).is_some() {
            self.current_project = key;
            match confy::store(&self.app_name, None, &self){
                Ok(_) => println!("app config successfully updated"),
                Err(e) => println!("there was an error updating project name: {:?}", e),
            };
        } else {
            println!("wha the flighting fuck")
        }
    }
}