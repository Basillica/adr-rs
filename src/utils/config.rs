use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::utils;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AppConfig {
    app_name: String,
    comfy: bool,
    foo: i64,
    current_adr_number: i64,
    default_path: PathBuf,
}

impl ::std::default::Default for AppConfig {
    fn default() -> Self {
        let app_name: &'static str = env!("CARGO_PKG_NAME");
        Self {
            app_name: app_name.to_string(),
            comfy: true,
            foo: 23.into(),
            current_adr_number: 0,
            default_path: "doc/adr".into(),
        }
    }
}

impl AppConfig{
    pub fn _update(&mut self, cfg: &AppConfig) {
        match confy::store(&self.app_name, None, cfg){
            Ok(_) => println!("app config successfully updated"),
            Err(e) => println!("there was an error updating app config. error: {:?}", e),
        };
    }

    pub fn doc_path(&mut self) -> &PathBuf {
        &self.default_path
    }

    pub fn next_adr_string(&mut self) -> String {
        utils::number_to_string(self.current_adr_number+1)
    }

    pub fn _current_adr_number(&mut self) -> i64 {
        self.current_adr_number
    }

    pub fn increment_count(&mut self) {
        self.current_adr_number += 1;
        match confy::store(&self.app_name, None, &self){
            Ok(_) => println!("app config successfully updated"),
            Err(e) => println!("there was an error incrementing app adr count. error: {:?}", e),
        };
    }
}