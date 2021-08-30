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
    pub fn new() -> Result<Self, &'static str> {
        let home_dir = match env::var("HOME") {
            Ok(home) => home,
            Err(e) => match e {
                VarError::NotPresent => return Err(
                    "HOME environment variable not found. \
                    Unable to find or create history file."
                ),
                VarError::NotUnicode(_) => return Err(
                    "Unable to parse HOME environment variable: \
                    Invalid unicode"
                ),
            },
        };

        let histfile = Path::new(&home_dir).join(".spacework_history");

        Ok(History { histfile })
    }

    pub fn write<'a>(&self, text: &'a str) -> Result<&'a str, Box<dyn Error>> {
        if !self.histfile.exists() {
            self.create_history_file()?;
        }
        
        let mut file = match OpenOptions::new()
            .append(true)
            .open(&self.histfile) {
            Ok(file) => file,
            Err(e) => return Err(
                format!("Unable to open history file: {}", e).into()
            ),
        };

        match file.write_all(
            format!("{} {}\n", self.timestamp(), text).as_bytes()
        ) {
            Ok(_) => Ok(text),
            Err(e) => Err(
                format!("Unable to write to history file: {}", e).into()
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

    fn create_history_file(&self) -> Result<(), Box<dyn Error>> {
        match File::create(&self.histfile) {
            Ok(_) => {
                println!(
                    "Created spacework history file: {}",
                    &self.histfile.display()
                );
                self.write("Hello hello, world!")?;
                Ok(())
            },
            Err(e) => Err(format!("Unable to create history file: {}", e).into())
        }
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

pub fn write(text: &str) -> Result<&str, Box<dyn Error>> {
    let history = History::new()?;

    history.write(text)
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
