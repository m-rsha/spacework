use std::env;
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
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let histfile =
            Path::new(&env::var("HOME")?).join(".spacework_history");

        Ok(History { histfile })
    }

    pub fn write<'a>(&self, text: &'a str) -> Result<&'a str, Box<dyn Error>> {
        if !self.histfile.exists() {
            self.create_history_file()?;
        }

        let mut file =
            match OpenOptions::new().append(true).open(&self.histfile) {
                Ok(file) => file,
                Err(e) => return Err(format!("Handle me: {}", e).into()),
            };

        match file
            .write_all(format!("{} {}\n", self.format_time(), text).as_bytes())
        {
            Ok(_) => (),
            Err(e) => return Err(format!("Handle me: {}", e).into()),
        };

        Ok(text)
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

    fn format_time(&self) -> String {
        Local::now().format("%Y-%m-%d@%X: ").to_string()
    }

    fn create_history_file(&self) -> Result<(), Box<dyn Error>> {
        File::create(&self.histfile)?;
        self.write("Hello hello, world!")?;
        // println!("Created spacework history file: {}", &histfile.display());

        Ok(())
    }

    fn delete_history_file(&self) -> Result<(), Box<dyn Error>> {
        match fs::remove_file(&self.histfile) {
            Ok(_) => Ok(()),
            Err(e) => match e.kind() {
                // Is it bad design to ignore this? I'm not sure
                ErrorKind::NotFound => Ok(()),
                _ => {
                    Err(format!("Unable to delete history file: {}", e).into())
                }
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
