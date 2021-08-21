use std::env;
use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str;

use chrono::prelude::*;

pub struct History {
    histfile: PathBuf,
}

impl History {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let histfile = Path::new(&env::var("HOME")?)
            .join(".spacework_history");

        Ok(History { histfile })
    }

    pub fn write<'a>(&self, text: &'a str) -> Result<&'a str, Box<dyn Error>> {
        if !self.histfile.exists() {
            self.create_history_file()?;
        }

        let mut file = OpenOptions::new().append(true).open(&self.histfile)?;
        writeln!(&file, "{} {}", self.format_time(), text)?;

        // Not sure if I need to call `flush`, but it was recommended
        file.flush()?;

        Ok(text)
    }

    pub fn read(&self) -> Result<String, Box<dyn Error>> {
        // TODO:
        // Print last few items.
        // Print specific actions, such as last n creations.
 
        let lines = fs::read_to_string(&self.histfile)?;
/*
        let lines: Vec<&str> = fs::read_to_string(&self.histfile)?
            .lines()
            .rev()
            .collect();
*/

        Ok(lines)
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
}

pub fn read_all() -> Result<String, Box<dyn Error>> {
    Ok(fs::read_to_string(History::new()?.histfile)?)
}

pub fn write(text: &str) -> Result<&str, Box<dyn Error>> {
    let history = History::new()?;

    history.write(text)
}

pub fn delete_all() -> Result<(), Box<dyn Error>> {
    let file = History::new()?.histfile;
    eprintln!("{:#?}", file);
    // fs::remove_file(History::new().histfile)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;
    
    #[test]
    fn todo() {
        assert!(true);
    }
}
