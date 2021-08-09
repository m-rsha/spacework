use super::language::Language;

use std::env;
use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Output;
use std::str;

use chrono::prelude::*;

#[allow(dead_code)]
fn log(data: &str) -> Result<(), Box<dyn Error>> {
    let mut histfile: PathBuf = env::var("HOME")?.parse()?;
    histfile.push(".spacework_history");
    if !histfile.exists() {
        File::create(&histfile)?;
        log("Hello hello, world!")?;
        println!("Created spacework history file: {}", &histfile.display());
    }

    let mut file = OpenOptions::new()
        .append(true)
        .open(&histfile)?;
    let time = Local::now().format("%Y-%m-%d@%X: ");
    let mut s = time.to_string();
    s.push_str(data);
    s.push('\n');
    file.write_all(s.as_bytes())?;
    // Not entirely sure if I need to call `flush`
    file.flush()?;
    Ok(())
}

pub fn create_workspace(
    name: Option<&str>,
    language: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let proj_name = match name {
        Some(name) => name,
        None => return Err("Workspace requires a name".into()),
    };
    let language = match language {
        Some(language) => match language {
            "c" => Language::C,
            "cpp" | "c++" => Language::Cpp,
            "py" | "python" => Language::Python,
            _ => return Err("Invalid language selection".into()),
        },
        None => return Err("Language required".into()),
    };

    let ws = workspace_home()?;
    if !ws.exists() {
        fs::create_dir_all(&ws)?;
        println!("Created `spacework` directory: {}", &ws.display());
    }

    let mut proj_dir = ws;
    match language {
        Language::C => proj_dir.push("c"),
        Language::Cpp => proj_dir.push("cpp"),
        Language::Python => proj_dir.push("py"),
    }
    proj_dir.push(proj_name);
    if proj_dir.exists() {
        return Err("Project directory already exists".into());
    }
    fs::create_dir_all(&proj_dir)?;
    println!("Created directory: {}", &proj_dir.display());
    
    let src_dir = &proj_dir.join("src");
    fs::create_dir_all(&src_dir)?;
    File::create(&src_dir.join(language.src_file()))?;

    let bin_dir = &proj_dir.join("bin");
    fs::create_dir_all(&bin_dir)?;

    Ok(())
}

pub fn compile() -> Result<Output, Box<dyn Error>> {
    if !is_inside_workspace()? {
        return Err("Not inside a `spacework` workspace".into());
    }
    let lang = get_language()?;
    eprintln!("We have a {:?} file!", lang);
    lang.compile()
}

fn get_language() -> Result<Language, Box<dyn Error>> {
    // This method feels so ugly. :c
    let mut extensions: Vec<String> = Vec::new();
    for directory in &[".", "./src"] {
        if Path::new(directory).exists() {
            for file in fs::read_dir(directory)? {
                let file = file?.path();
                if let Some(ext) = file.extension() {
                    if let Some(ext) = ext.to_str() {
                        extensions.push(ext.to_string());
                    }
                }
            }
        }
    }

    for ext in extensions.iter() {
        eprintln!("{}", &ext);
        if ext == "c" {
            return Ok(Language::C);
        } else if ext == "cpp" {
            return Ok(Language::Cpp);
        } else if ext == "py" {
            return Ok(Language::Python);
        }
    }
    Err("Found no files to compile".into())
}

pub fn print_history() -> Result<(), Box<dyn Error>> {
    // TODO:
    // Print last few items.
    // Print specific actions, such as last n creations.
    // Probably need to figure out how to use `Seek` and
    // `SeekFrom::End()`
    let file = fs::read_to_string(
        Path::new(&env::var("HOME")?)
        .join(".spacework_history")
    )?;
    print!("{}", &file);
    Ok(())
}
    
fn is_inside_workspace() -> Result<bool, Box<dyn Error>> {
    Ok(env::current_dir()?.starts_with(workspace_home()?))
}

fn workspace_home() -> Result<PathBuf, Box<dyn Error>> {
    Ok(Path::new(&env::var("HOME")?).join("spacework"))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn detects_inside_workspace_dir() -> Result<(), Box<dyn Error>> {
        env::set_current_dir(env::temp_dir())?;
        assert!(!is_inside_workspace()?);

        env::set_current_dir(&workspace_home()?)?;
        assert!(is_inside_workspace()?);
        
        Ok(())
    }
}
