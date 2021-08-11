use std::env;
use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::str;
use chrono::prelude::*;

// use std::process::Output;

fn history_file() -> Result<PathBuf, Box<dyn Error>> {
    Ok(env::var("HOME")?.parse::<PathBuf>()?.join(".spacework_history"))
}

pub fn write(text: &str) -> Result<(), Box<dyn Error>> {
    let histfile = history_file()?;
    if !histfile.exists() {
        File::create(&histfile)?;
        write("Hello hello, world!")?;
        println!("Created spacework history file: {}", &histfile.display());
    }
    let mut file = OpenOptions::new().append(true).open(histfile)?;
    let time = Local::now().format("%Y-%m-%d@%X: ").to_string();
    writeln!(&file, "{}: {}", time, text)?;
    // Not entirely sure if I need to call `flush`
    file.flush()?;

    Ok(())
}

pub fn read() -> Result<(), Box<dyn Error>> {
    // TODO:
    // Print last few items.
    // Print specific actions, such as last n creations.
    // Probably need to figure out how to use `Seek` and
    // `SeekFrom::End()`
    let file = fs::read_to_string(history_file()?)?;
    print!("{}", &file);

    Ok(())
}

/*
pub fn append(data: &str) -> Result<(), Box<dyn Error>> {
    let histfile = OpenOptions::new().append(true).open(history_file()?)?;

    Ok(())
}

*/

    
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn todo() {
        assert!(true);
    }
}
