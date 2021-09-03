use std::env::{self, VarError};
use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::str;

use chrono::prelude::*;

pub struct History {
    histfile: PathBuf,
}

impl History {
    const HISTFILE: &'static str = ".spacework_history";

    pub fn new() -> Result<Self, Box<dyn Error>> {
        let home_dir = match env::var("HOME") {
            Ok(home) => home,
            Err(e) => match e {
                VarError::NotPresent => return Err(
                    "HOME environment variable not found. \
                    Unable to find or create history file".into()
                ),
                VarError::NotUnicode(_) => return Err(
                    "Unable to parse HOME environment variable: \
                    Invalid unicode".into()
                ),
            },
        };
        
        let histfile = Path::new(&home_dir).join(Self::HISTFILE);
        if !histfile.exists() {
            Self::create_history_file(&histfile)?;
        }

        Ok(History { histfile })
    }

    pub fn write<'a>(&self, text: &'a str) -> Result<&'a str, Box<dyn Error>> {
        match self
            .file()?
            .write_all(
            format!("{} {}\n", self.timestamp(), text).as_bytes())
        {
            Ok(_) => Ok(text),
            Err(e) => Err(
                format!("Unable to write to history file: {}", e).into()
            ),
        }
    }
    
    fn create_history_file(filepath: &Path) -> Result<(), Box<dyn Error>> {
        match File::create(filepath) {
            Ok(_) => {
                Self::write(
                    &Self{ histfile: filepath.into() }, 
                    "Hello hello, world!",
                )?;
                println!(
                    "Created spacework history file: {}",
                    filepath.display()
                );
                Ok(())
            },
            Err(e) => Err(
                format!("Unable to create history file: {}", e).into()
            ),
        }
    }

    pub fn file(&self) -> Result<File, Box<dyn Error>> {
        match OpenOptions::new().append(true).open(&self.histfile) {
            Ok(file) => Ok(file),
            Err(e) => return Err(
                format!("Unable to open history file: {}", e).into()
            ),
        }
    }
    
    pub fn read_last(
        &self,
        last: usize,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        Ok(fs::read_to_string(&self.histfile)?
            .lines()
            .rev()
            .take(last)
            .map(|s| s.to_string())
            .collect())
    }

    pub fn read_all(&self) -> Result<String, Box<dyn Error>> {
        Ok(fs::read_to_string(&self.histfile)?)
    }

    fn timestamp(&self) -> String {
        Local::now().format("%Y-%m-%d@%X: ").to_string()
    }

    fn delete_history_file(&self) -> Result<(), Box<dyn Error>> {
        match fs::remove_file(&self.histfile) {
            Ok(_) => Ok(()),
            Err(e) => match e.kind() {
                // Is it bad design to ignore this? I'm not sure
                ErrorKind::NotFound => Ok(()),
                _ => Err(format!(
                    "Unable to delete history file: {}", e).into()
                ),
            },
        }
    }
}

pub fn delete_history_file() -> Result<(), Box<dyn Error>> {
    History::new()?.delete_history_file()
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn todo() {
        assert!(true);
    }
}
