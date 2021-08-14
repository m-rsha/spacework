use serde::Deserialize;

use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct SpaceworkFile {
    pub workspace: Workspace,
}

#[derive(Debug, Deserialize)]
pub struct Workspace {
    pub language: String,
}

// const DEFAULT_TOML: &'static str = include_str!("../../spacework.toml");

impl SpaceworkFile {
    pub fn from_str(sw_file: &str) -> Result<Self, Box<dyn Error>> {
        Ok(toml::from_str(sw_file)?)
    }
}
