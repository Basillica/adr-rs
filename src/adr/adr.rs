use chrono::Utc;
use serde::{Deserialize, Serialize};


use crate::utils;

#[derive(Serialize, Deserialize, Debug)]
pub struct Adr {
    pub id: String,
    pub title: String,
    pub description: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

pub trait Builder{
    fn new() -> Self;
    fn set_title(&mut self, title: &str) -> &mut Self;
    fn set_description(&mut self, description: &str) -> &mut Self;
    fn set_content(&mut self, content: &str) -> &mut Self;
    fn build(&self) -> Adr;
}

impl Builder for Adr {
    fn new() -> Self {
        let now = match utils::now() {
            Ok(v) => v,
            Err(_) => Utc::now().to_string(),
        };
        Adr {
            id: utils::generate_unique_id(),
            title: String::new(),
            description: String::new(),
            content: String::new(),
            created_at: now.clone(),
            updated_at: now,
        }
    }

    fn set_title(&mut self, title: &str) -> &mut Self {
        self.title = title.to_owned();
        self
    }

    fn set_description(&mut self, description: &str) -> &mut Self {
        self.description = description.to_owned();
        self
    }

    fn set_content(&mut self, content: &str) -> &mut Self {
        self.content = content.to_owned();
        self
    }

    fn build(&self) -> Adr {
        Adr {
            id: utils::generate_unique_id(),
            title: self.title.to_owned(),
            description: self.description.to_owned(),
            content: self.content.to_owned(),
            created_at: self.created_at.to_owned(),
            updated_at: self.updated_at.to_owned(),
        }
    }
}