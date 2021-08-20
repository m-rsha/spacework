use crate::spacework::workspace;
use crate::config::languagefile::LanguageFile;

use serde::Deserialize;

use std::error::Error;
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::Write;

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

    pub fn find_in_dir(dir: &mut PathBuf) -> Result<Self, Box<dyn Error>> {
        if !workspace::is_inside_workspace(dir)? {
            Err("Must be inside a spacework workspace".into())
        } else if let Ok(cfg) = fs::read_to_string("spacework.toml") {
            Ok(Self::from_str(&cfg)?)
        } else if dir.pop() {
            Self::find_in_dir(dir)
        } else {
            Err("`spacework.toml` file not found".into())
        }
    }

    pub fn create(
        dir: &Path,
        langfile: &LanguageFile
    ) -> Result<File, Box<dyn Error>> {
        let mut cfg = File::create(dir.join("spacework.toml"))?;
        let toml = format!(
            "[workspace]\nlanguage = \"{}\"", langfile.language.name
        );
        cfg.write_all(toml.as_bytes())?;

        Ok(cfg)
    }
}
